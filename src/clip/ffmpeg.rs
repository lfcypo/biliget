use std::path::Path;
use std::process::Command;

pub fn merge_video(video_file: &Path, audio_file: &Path, output_file: &Path) {
    if !Path::new(&video_file).exists() || !Path::new(&audio_file).exists() {
        return;
    }

    Command::new("ffmpeg")
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
        .unwrap();
}
