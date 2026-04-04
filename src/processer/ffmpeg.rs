use crate::concat_arrays;
use crate::processer::ffmpeg::FfmpegError::FileNameError;
use crate::processer::progress_bar::FfmpegProgress;
use crate::progress::bar::Bar;
use std::path::Path;
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

const FFMPEG_GLOBAL_ARGS: [&str; 4] = ["-hide_banner", "-progress", "pipe:1", "-nostats"];

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

async fn get_media_duration_us(path: &Path) -> Result<u64, FfmpegError> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str().ok_or(FfmpegError::FileNameError())?,
        ])
        .output()
        .await
        .map_err(|e| FfmpegError::ConvertError(format!("无法启动 ffprobe: {}", e)))?;

    if !output.status.success() {
        return Ok(0);
    }

    let duration_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let duration_secs: f64 = duration_str.parse().unwrap_or(0.0);

    Ok((duration_secs * 1_000_000.0) as u64)
}

async fn handle_ffmpeg_progress(
    mut child: tokio::process::Child,
    mut progress_bar: impl Bar,
    cancel: CancellationToken,
    err_map: impl Fn(String) -> FfmpegError,
) -> Result<(), FfmpegError> {
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| err_map("无法获取 FFmpeg stdout".to_string()))?;

    let mut reader = BufReader::new(stdout).lines();
    let mut progress_lines = Vec::with_capacity(12);

    let status = tokio::select! {
        res = async {
            while let Ok(Some(line)) = reader.next_line().await {
                progress_lines.push(line);

                if progress_lines.last().map(|s| s.starts_with("progress=")).unwrap_or(false) {
                    let lines_ref: Vec<&str> = progress_lines.iter().map(|s| s.as_str()).collect();
                    let prog = FfmpegProgress::from_lines(&lines_ref);
                    progress_bar.set_progress(prog.out_time_us).await;
                    progress_lines.clear();
                }
            }
            let wait_res = child.wait().await;
            progress_bar.finish().await;
            wait_res
        } => res,

        () = cancel.cancelled() => {
            let _ = child.kill().await;
            progress_bar.finish().await;
            return Err(FfmpegError::Cancelled);
        }
    };

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(err_map(format!("Exit code: {}", s.code().unwrap_or(-1)))),
        Err(e) => Err(err_map(e.to_string())),
    }
}

pub(crate) async fn merge_video(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
    mut progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), FfmpegError> {
    if !video_file.exists() || !audio_file.exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let duration = get_media_duration_us(audio_file).await?;
    progress_bar.set_length(duration).await;

    let child = Command::new("ffmpeg")
        .args(concat_arrays!(
            FFMPEG_GLOBAL_ARGS,
            [
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
            ]
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| FfmpegError::MergeError(format!("启动失败: {}", e)))?;

    handle_ffmpeg_progress(child, progress_bar, cancel, FfmpegError::MergeError).await
}

pub(crate) async fn convert_audio(
    audio_file: &Path,
    output_file: &Path,
    mut progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), FfmpegError> {
    if !audio_file.exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let duration = get_media_duration_us(audio_file).await?;
    progress_bar.set_length(duration).await;

    let child = Command::new("ffmpeg")
        .args(concat_arrays!(
            FFMPEG_GLOBAL_ARGS,
            [
                "-i",
                audio_file.to_str().ok_or_else(FileNameError)?,
                "-vn",
                "-ar",
                "44100",
                "-ac",
                "2",
                "-y",
                output_file.to_str().ok_or_else(FileNameError)?,
            ]
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| FfmpegError::ConvertError(format!("启动 ffmpeg 失败: {}", e)))?;

    handle_ffmpeg_progress(child, progress_bar, cancel, FfmpegError::ConvertError).await
}
