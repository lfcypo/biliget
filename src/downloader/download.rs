use crate::bar::Bar;
use crate::util::header::to_hashmap;
use crate::util::size_fmt::format_size;
use crate::util::space::check_free_space;
use fast_down_ffi as fd;
use fast_down_ffi::Total;
use http::HeaderMap;
use std::fmt::Debug;
use std::path::Path;
use std::time::Duration;
use thiserror::Error;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use url::Url;

const MAX_RETRY_TIMES: usize = 3;
const RETRY_GAP: Duration = Duration::from_millis(500);
const TICK_RATE: Duration = Duration::from_millis(100);

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum DownloaderError {
    #[error("文件路径错误")]
    FilePathError(),

    #[error("磁盘空间不足: {0}")]
    InsufficientDiskSpaceError(String),

    #[error("解析 URL 错误: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("下载错误")]
    DownloadError(#[source] Box<fd::Error>),
}

impl From<fd::Error> for DownloaderError {
    fn from(error: fd::Error) -> Self {
        Self::DownloadError(Box::new(error))
    }
}

pub async fn download_file(
    url: &str,
    dest_path: &Path,
    headers: &HeaderMap,
    mut progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), DownloaderError> {
    let url = Url::parse(url)?;
    let parent = dest_path.parent().ok_or(DownloaderError::FilePathError())?;

    let fd_config = fd::Config {
        headers: to_hashmap(headers),
        retry_times: MAX_RETRY_TIMES,
        retry_gap: RETRY_GAP,
        ..Default::default()
    };

    let (tx, rx) = fd::create_channel();
    let task = fd::prefetch(url, fd_config, tx).await?;

    let disk_usage = task.info.size;
    if let Some(size) = check_free_space(parent, disk_usage).map_err(|_| {
        DownloaderError::InsufficientDiskSpaceError("获取磁盘剩余空间失败".to_string())
    })? {
        return Err(DownloaderError::InsufficientDiskSpaceError(format!(
            "还需要 {}",
            format_size(size as f64)
        )));
    }

    progress_bar.set_length(task.info.size).await;
    let download_task = task.start(dest_path.to_path_buf(), cancel.clone());

    let mut last_tick = Instant::now();
    let mut pending_progress = 0u64;

    tokio::select! {
        res = download_task => {
            res?;
        }
        _ = async {
            while let Ok(event) = rx.recv().await {
                match event {
                    fd::Event::PushProgress(_, progress_entry) => {
                        pending_progress += progress_entry.total();

                        if last_tick.elapsed() >= TICK_RATE {
                            progress_bar.update_progress(pending_progress).await;
                            pending_progress = 0;
                            last_tick = Instant::now();
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            if pending_progress > 0 {
                progress_bar.update_progress(pending_progress).await;
            }
            progress_bar.finish().await;
        } => {}
    }

    Ok(())
}
