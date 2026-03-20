use crate::processer::ffmpeg::FfmpegError::FileNameError;
use std::path::Path;
use thiserror::Error;
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Error)]
pub(crate) enum FfmpegError {
    #[error("找不到相关文件")]
    FileNotFound(),

    #[error("文件名错误")]
    FileNameError(),

    #[error("合并音视频失败: {0}")]
    MergeError(String),

    #[error("格式转换失败: {0}")]
    ConvertError(String),

    #[error("操作已取消")]
    Cancelled,
}

pub(crate) async fn merge_video(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
    cancel: CancellationToken,
) -> Result<(), FfmpegError> {
    if !video_file.exists() || !audio_file.exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            video_file.to_str().ok_or_else(FileNameError)?,
            "-i",
            audio_file.to_str().ok_or_else(FileNameError)?,
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-y",
            output_file.to_str().ok_or_else(FileNameError)?,
        ])
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| FfmpegError::MergeError(format!("启动 ffmpeg 失败: {}", e)))?;

    let status = tokio::select! {
        () = cancel.cancelled() => {
            let _ = child.kill().await;
            return Err(FfmpegError::Cancelled);
        }
        status = child.wait() => status,
    };

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(FfmpegError::MergeError(format!(
            "ffmpeg 执行失败 exit code: {}",
            status.code().unwrap_or(-1)
        ))),
        Err(e) => Err(FfmpegError::MergeError(format!(
            "等待 ffmpeg 进程失败: {}",
            e
        ))),
    }
}

pub(crate) async fn convert_audio(
    audio_file: &Path,
    output_file: &Path,
    cancel: CancellationToken,
) -> Result<(), FfmpegError> {
    if !audio_file.exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            audio_file.to_str().ok_or_else(FileNameError)?,
            "-vn",
            "-ar",
            "44100",
            "-ac",
            "2",
            "-y",
            output_file.to_str().ok_or_else(FileNameError)?,
        ])
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| FfmpegError::ConvertError(format!("启动 ffmpeg 失败: {}", e)))?;

    let status = tokio::select! {
        () = cancel.cancelled() => {
            let _ = child.kill().await;
            return Err(FfmpegError::Cancelled);
        }
        status = child.wait() => status,
    };

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(FfmpegError::ConvertError(format!(
            "ffmpeg 执行失败 exit code: {}",
            status.code().unwrap_or(-1)
        ))),
        Err(e) => Err(FfmpegError::ConvertError(format!(
            "等待 ffmpeg 进程失败: {}",
            e
        ))),
    }
}
