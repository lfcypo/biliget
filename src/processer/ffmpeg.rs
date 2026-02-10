use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum FfmpegError {
    #[error("找不到相关文件")]
    FileNotFound(),

    #[error("合并音视频失败: {0}")]
    MergeError(String),

    #[error("格式转换失败: {0}")]
    ConvertError(String),
}

pub(crate) fn merge_video(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
) -> Result<(), FfmpegError> {
    if !Path::new(&video_file).exists() || !Path::new(&audio_file).exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let status = match Command::new("ffmpeg")
        .args([
            "-i",
            video_file.to_str().unwrap(),
            "-i",
            audio_file.to_str().unwrap(),
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-y",
            output_file.to_str().unwrap(),
        ])
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            return Err(FfmpegError::MergeError(
                e.to_string().as_str().parse().unwrap(),
            ));
        }
    };

    if !status.success() {
        return Err(FfmpegError::MergeError("ffmpeg执行失败".to_string()));
    }

    Ok(())
}

pub(crate) fn convert_audio(audio_file: &Path, output_file: &Path) -> Result<(), FfmpegError> {
    if !Path::new(&audio_file).exists() {
        return Err(FfmpegError::FileNotFound());
    }

    let status = match Command::new("ffmpeg")
        .args([
            "-i",
            audio_file.to_str().unwrap(),
            "-vn",
            "-ar",
            "44100",
            "-ac",
            "2",
            "-y",
            output_file.to_str().unwrap(),
        ])
        .status()
    {
        Ok(status) => status,
        Err(e) => {
            return Err(FfmpegError::ConvertError(
                e.to_string().as_str().parse().unwrap(),
            ));
        }
    };

    if !status.success() {
        return Err(FfmpegError::ConvertError("ffmpeg执行失败".to_string()));
    }

    Ok(())
}
