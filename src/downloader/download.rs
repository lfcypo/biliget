use crate::util::size_fmt::format_size;
use crate::util::space::check_free_space;
use fast_down::file::{FilePusher, MmapFilePusher};
use fast_down::http::Prefetch;
use fast_down::multi::download_multi;
use fast_down::single::download_single;
use fast_down::utils::{FastDownPuller, FastDownPullerOptions, build_client};
use fast_down::{BoxPusher, Event, Total, multi, single};
use http::HeaderMap;
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use std::time;
use thiserror::Error;
use tokio::fs;
use tokio::fs::OpenOptions;
use url::Url;

const MAX_RETRY_TIMES: usize = 3;
const RETRY_DURATION: time::Duration = time::Duration::from_millis(500);

const PROXY: &str = "";

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("文件路径错误")]
    FilePathError(),

    #[error("磁盘空间不足: {0}")]
    InsufficientDiskSpaceError(String),

    #[error("解析url错误: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("请求错误: {0}")]
    RequestError(String),

    #[error("写入文件错误")]
    WriteError(String),
}

// pub fn download_file(
//     url: &String,
//     dest_file: &PathBuf,
//     headers: &HeaderMap,
// ) -> Result<(), DownloadError> {
//     let client = reqwest::blocking::Client::new();
//     let mut response = client.get(url).headers(headers.clone()).send()?;
//     let mut file = std::fs::File::create(dest_file)?;
//     std::io::copy(&mut response, &mut file)?;
//
//     Ok(())
// }

pub async fn download_file(
    url: &str,
    dest: &PathBuf,
    headers: &HeaderMap,
) -> Result<(), DownloadError> {
    let url = Url::parse(url)?;
    let parent = dest.parent().ok_or(DownloadError::FilePathError())?;

    let client = build_client(headers, PROXY, false, false)
        .map_err(|err| DownloadError::RequestError(err.to_string()))?;

    let mut retry_times = 0;
    let (info, resp) = loop {
        if retry_times > MAX_RETRY_TIMES {
            return Err(DownloadError::RequestError(
                "下载失败，请检查网络".to_string(),
            ));
        }
        match client.prefetch(url.clone()).await {
            Ok(info) => break info,
            Err(_) => retry_times += 1,
        }
        tokio::time::sleep(RETRY_DURATION).await;
    };

    let threads = if info.fast_download { 32 } else { 1 };
    #[allow(clippy::single_range_in_vec_init)]
    let download_chunks = vec![0..info.size];
    let push_queue_cap = 10 * 1024;
    let write_buffer_size = 8 * 1024 * 1024;

    if let Some(size) = check_free_space(parent, download_chunks.total()).map_err(|_| {
        DownloadError::InsufficientDiskSpaceError("获取磁盘剩余空间失败".to_string())
    })? {
        return Err(DownloadError::InsufficientDiskSpaceError(format!(
            "还需要 {}",
            format_size(size as f64)
        )));
    }

    let puller = FastDownPuller::new(FastDownPullerOptions {
        url: info.final_url,
        proxy: PROXY,
        multiplexing: false,
        headers: Arc::new(headers.clone()),
        accept_invalid_certs: false,
        accept_invalid_hostnames: false,
        file_id: info.file_id.clone(),
        resp: Some(Arc::new(Mutex::new(Some(resp)))),
    })
    .map_err(|err| DownloadError::RequestError(err.to_string()))?;
    if let Err(err) = fs::create_dir_all(parent).await {
        return Err(DownloadError::WriteError(err.to_string()));
    }
    let result = if info.fast_download {
        let pusher = MmapFilePusher::new(&dest, info.size)
            .await
            .map_err(|err| DownloadError::RequestError(err.to_string()))?;
        let pusher = BoxPusher::new(pusher);
        download_multi(
            puller,
            pusher,
            multi::DownloadOptions {
                download_chunks: download_chunks.iter(),
                retry_gap: RETRY_DURATION,
                concurrent: threads,
                push_queue_cap,
                min_chunk_size: 8 * 1024,
            },
        )
    } else {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(dest)
            .await
            .map_err(|err| DownloadError::WriteError(err.to_string()))?;
        let pusher = FilePusher::new(file, info.size, write_buffer_size)
            .await
            .map_err(|err| DownloadError::WriteError(err.to_string()))?;
        let pusher = BoxPusher::new(pusher);
        download_single(
            puller,
            pusher,
            single::DownloadOptions {
                retry_gap: RETRY_DURATION,
                push_queue_cap,
            },
        )
    };

    let result_clone = result.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        result_clone.abort();
    });

    while let Ok(e) = result.event_chain.recv().await {
        match e {
            Event::PullError(_, _) => {
                return Err(DownloadError::RequestError("请求失败".to_string()));
            }
            Event::PushError(_, _) | Event::FlushError(_) => {
                return Err(DownloadError::WriteError("写入文件失败".to_string()));
            }
            _ => {}
        }
    }

    Ok(())
}
