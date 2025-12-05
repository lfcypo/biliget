use http::HeaderMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("请求错误: {0}")]
    Request(#[from] reqwest::Error),

    #[error("写入文件错误")]
    WriteError(#[from] std::io::Error),
}

pub fn download_file(url: &String, dest_file: &PathBuf, headers: &HeaderMap) -> Result<(), DownloadError> {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).headers(headers.clone()).send()?;
    let mut file = std::fs::File::create(dest_file)?;
    std::io::copy(&mut response, &mut file)?;

    Ok(())
}
