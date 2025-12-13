use crate::processer::ffmpeg::{FfmpegError, convert_audio, merge_video};
use std::path::Path;
use thiserror::Error;

pub(crate) struct ProcessOption<'a> {
    pub video_file: Option<&'a Path>,
    pub audio_file: Option<&'a Path>,
    pub output_file: &'a Path,

    pub only_audio: bool,
}

#[derive(Debug, Error)]
pub enum ProcessError {
    #[error("后处理错误")]
    ProcessError(#[from] FfmpegError),
}

pub fn process(option: ProcessOption) -> Result<(), ProcessError> {
    if option.only_audio {
        process_only_audio(option.audio_file.unwrap(), option.output_file)?
    } else {
        process_default(
            Path::new(&option.video_file.unwrap()),
            Path::new(&option.audio_file.unwrap()),
            Path::new(&option.output_file),
        )?
    }

    Ok(())
}

fn process_default(
    video_file: &Path,
    audio_file: &Path,
    output_file: &Path,
) -> Result<(), ProcessError> {
    Ok(merge_video(video_file, audio_file, output_file)?)
}

fn process_only_audio(audio_file: &Path, output_file: &Path) -> Result<(), ProcessError> {
    Ok(convert_audio(audio_file, output_file)?)
}
