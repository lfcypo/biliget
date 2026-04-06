use crate::bar::Bar;
use crate::processer::ffmpeg::{FfmpegError, convert_audio, merge_video};
use std::path::Path;
use thiserror::Error;
use tokio_util::sync::CancellationToken;

pub(crate) struct ProcessOption<'a> {
    pub video_file: Option<&'a Path>,
    pub audio_file: Option<&'a Path>,
    pub output_file: &'a Path,

    pub only_audio: bool,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("参数错误")]
    ParamError(),

    #[error("后处理错误")]
    ProcessError(#[from] FfmpegError),

    #[error("操作已取消")]
    Cancelled(),
}

pub async fn process(
    option: ProcessOption<'_>,
    progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), ProcessError> {
    if option.only_audio {
        process_only_audio(
            option.audio_file.ok_or_else(ProcessError::ParamError)?,
            option.output_file,
            progress_bar,
            cancel,
        )
        .await?
    } else {
        process_default(
            Path::new(&option.video_file.ok_or_else(ProcessError::ParamError)?),
            Path::new(&option.audio_file.ok_or_else(ProcessError::ParamError)?),
            Path::new(&option.output_file),
            progress_bar,
            cancel,
        )
        .await?
    }

    Ok(())
}

async fn process_default(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
    progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), ProcessError> {
    merge_video(video_file, audio_file, output_file, progress_bar, cancel)
        .await
        .map_err(|e| match e {
            FfmpegError::Cancelled => ProcessError::Cancelled(),
            _ => ProcessError::ProcessError(e),
        })
}

async fn process_only_audio(
    audio_file: &Path,
    output_file: &Path,
    progress_bar: impl Bar,
    cancel: CancellationToken,
) -> Result<(), ProcessError> {
    convert_audio(audio_file, output_file, progress_bar, cancel)
        .await
        .map_err(|e| match e {
            FfmpegError::Cancelled => ProcessError::Cancelled(),
            _ => ProcessError::ProcessError(e),
        })
}
