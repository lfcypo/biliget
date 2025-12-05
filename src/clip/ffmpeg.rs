use std::path::Path;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
    #[error("找不到相关文件")]
    FileNotFound(),

    #[error("合并音视频失败: {0}")]
    MergeError(String),
}

pub fn merge_video(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
) -> Result<(), ClipError> {
    if !Path::new(&video_file).exists() || !Path::new(&audio_file).exists() {
        return Err(ClipError::FileNotFound());
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
            "-b:a",
            "192k",
            "-y",
            output_file.to_str().unwrap(),
        ])
        .status()
    {
        Ok(status) => status,
        Err(e) => return Err(ClipError::MergeError(e.to_string().as_str().parse().unwrap())),
    };

    if !status.success() {
        return Err(ClipError::MergeError("ffmpeg执行失败".to_string()))
    }

    Ok(())
}
