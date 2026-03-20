use crate::util::header::to_hashmap;
use crate::util::size_fmt::format_size;
use crate::util::space::check_free_space;
use fast_down_ffi as fd;
use http::HeaderMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time;
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use url::Url;

const MAX_RETRY_TIMES: usize = 3;
const RETRY_GAP: time::Duration = time::Duration::from_millis(500);

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum DownloaderError {
    #[error("文件路径错误")]
    FilePathError(),

    #[error("磁盘空间不足: {0}")]
    InsufficientDiskSpaceError(String),

    #[error("解析url错误: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("请求错误")]
    DownloadError(#[from] fd::Error),
}

pub async fn download_file(
    url: &str,
    dest_path: &Path,
    headers: &HeaderMap,
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

    let (tx, _rx) = fd::create_channel();
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

    task.start(PathBuf::from(dest_path), cancel).await?;

    Ok(())
}
