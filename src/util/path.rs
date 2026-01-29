use crate::cli::Cli;
use sanitize_filename::sanitize;
use std::fs;
use std::path::{Path, PathBuf};

fn get_current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|err| {
        println!("获取运行目录失败喵: {err}");
        std::process::exit(1);
    })
}

fn get_temp_paths(base_dir: &Path, file_name: &str) -> (PathBuf, PathBuf) {
    let name = if let Some((before_dot, _)) = file_name.split_once('.') {
        before_dot
    } else {
        file_name
    };
    let video_temp_file = base_dir.join(format!("{name}-video.tmp"));
    let audio_temp_file = base_dir.join(format!("{name}-audio.tmp"));
    (video_temp_file, audio_temp_file)
}

fn get_output_file(base_dir: &Path, file_name: &str, is_audio: bool) -> PathBuf {
    if file_name.contains(".") {
        return base_dir.join(file_name);
    }
    let extension = if is_audio { "wav" } else { "mp4" };
    base_dir.join(format!("{file_name}.{extension}"))
}

pub fn get_paths(title: &str, cmd_option: &Cli) -> (PathBuf, PathBuf, PathBuf) {
    let sanitized_title = sanitize(title);

    if cmd_option.output.is_none() {
        let base_dir = get_current_dir();
        let output_file = get_output_file(&base_dir, &sanitized_title, cmd_option.only_audio);
        let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, &sanitized_title);
        return (output_file, video_temp_file, audio_temp_file);
    }

    let output_path = PathBuf::from(cmd_option.output.as_ref().unwrap());

    if output_path.is_absolute() {
        return handle_absolute_path(&output_path, &sanitized_title, cmd_option.only_audio);
    }

    handle_relative_path(&output_path, &sanitized_title, cmd_option.only_audio)
}

fn handle_absolute_path(
    output_path: &PathBuf,
    title: &str,
    only_audio: bool,
) -> (PathBuf, PathBuf, PathBuf) {
    if output_path.try_exists().is_ok() {
        return if output_path.is_file() {
            let base_dir = output_path.parent().unwrap_or(Path::new("/")).to_path_buf();
            let output_file = output_path.to_path_buf();

            let file_stem = output_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(title);

            let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, file_stem);
            (output_file, video_temp_file, audio_temp_file)
        } else {
            let (base_dir, file_name) = if output_path.extension().is_some() {
                (
                    output_path.parent().unwrap_or(Path::new("/")).to_path_buf(),
                    output_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                (output_path.to_path_buf(), title.to_string())
            };
            ensure_directory_exists(&base_dir);
            let output_file = get_output_file(&base_dir, &file_name, only_audio);
            let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, &file_name);
            (output_file, video_temp_file, audio_temp_file)
        };
    }

    if has_file_extension(output_path) {
        let base_dir = output_path.parent().unwrap_or(Path::new("/")).to_path_buf();
        ensure_directory_exists(&base_dir);
        let output_file = output_path.to_path_buf();

        let file_stem = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(title);

        let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, file_stem);
        (output_file, video_temp_file, audio_temp_file)
    } else {
        ensure_directory_exists(output_path);
        let base_dir = output_path;
        let output_file = get_output_file(base_dir, title, only_audio);
        let (video_temp_file, audio_temp_file) = get_temp_paths(base_dir, title);
        (output_file, video_temp_file, audio_temp_file)
    }
}

fn handle_relative_path(
    output_path: &Path,
    title: &str,
    only_audio: bool,
) -> (PathBuf, PathBuf, PathBuf) {
    let abs_output_path = get_current_dir().join(output_path);

    if abs_output_path.try_exists().is_ok() {
        return if abs_output_path.is_file() {
            let current_dir = get_current_dir().to_path_buf();
            let abs_output_path_clone = abs_output_path.clone();
            let base_dir = abs_output_path_clone
                .parent()
                .unwrap_or(&current_dir)
                .to_path_buf();
            let output_file = abs_output_path;

            let file_stem = output_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(title);

            let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, file_stem);
            (output_file, video_temp_file, audio_temp_file)
        } else {
            let (base_dir, file_name) = if output_path.extension().is_some() {
                (
                    abs_output_path
                        .parent()
                        .unwrap_or(Path::new("/"))
                        .to_path_buf(),
                    abs_output_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                (abs_output_path.to_path_buf(), title.to_string())
            };
            ensure_directory_exists(&base_dir);
            let output_file = get_output_file(&base_dir, &file_name, only_audio);
            let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, &file_name);
            (output_file, video_temp_file, audio_temp_file)
        };
    }

    if has_file_extension(output_path) {
        let current_dir = get_current_dir().to_path_buf();
        let base_dir = abs_output_path
            .parent()
            .unwrap_or(&current_dir)
            .to_path_buf();
        ensure_directory_exists(&base_dir);
        let abs_output_path_clone = abs_output_path.clone();
        let output_file = abs_output_path_clone;

        let file_stem = output_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(title);

        let (video_temp_file, audio_temp_file) = get_temp_paths(&base_dir, file_stem);
        (output_file, video_temp_file, audio_temp_file)
    } else {
        ensure_directory_exists(&abs_output_path);
        let base_dir = &abs_output_path;
        let output_file = get_output_file(base_dir, title, only_audio);
        let (video_temp_file, audio_temp_file) = get_temp_paths(base_dir, title);
        (output_file, video_temp_file, audio_temp_file)
    }
}

fn ensure_directory_exists(dir: &PathBuf) {
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap_or_else(|err| {
            println!("创建目录失败喵: {err}");
            std::process::exit(1);
        });
    }
}

fn has_file_extension(path: &Path) -> bool {
    path.extension().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;

    fn create_cli_demo(output: Option<String>, only_audio: bool) -> Cli {
        Cli {
            url: "https://example.com".to_string(),
            output,
            only_audio,
        }
    }

    #[test]
    fn test_no_output_option() {
        let cli = create_cli_demo(None, false);
        let (output, video_temp, audio_temp) = get_paths("test video", &cli);

        assert!(output.ends_with("test video.mp4"));
        assert!(video_temp.ends_with("test video-video.tmp"));
        assert!(audio_temp.ends_with("test video-audio.tmp"));
    }

    #[test]
    fn test_no_output_option_audio() {
        let cli = create_cli_demo(None, true);
        let (output, video_temp, audio_temp) = get_paths("test audio", &cli);

        assert!(output.ends_with("test audio.wav"));
        assert!(video_temp.ends_with("test audio-video.tmp"));
        assert!(audio_temp.ends_with("test audio-audio.tmp"));
    }

    #[test]
    fn test_absolute_directory() {
        let cli = create_cli_demo(Some("/tmp".to_string()), false);
        let (output, video_temp, audio_temp) = get_paths("my video", &cli);

        assert_eq!(output, PathBuf::from("/tmp/my video.mp4"));
        assert_eq!(video_temp, PathBuf::from("/tmp/my video-video.tmp"));
        assert_eq!(audio_temp, PathBuf::from("/tmp/my video-audio.tmp"));
    }

    #[test]
    fn test_relative_directory() {
        let cli = create_cli_demo(Some("downloads".to_string()), false);
        let (output, video_temp, audio_temp) = get_paths("video title", &cli);

        let current_dir = get_current_dir();
        let expected_dir = current_dir.join("downloads");
        assert_eq!(output, expected_dir.join("video title.mp4"));
        assert_eq!(video_temp, expected_dir.join("video title-video.tmp"));
        assert_eq!(audio_temp, expected_dir.join("video title-audio.tmp"));
    }

    #[test]
    fn test_absolute_file_with_extension() {
        let cli = create_cli_demo(Some("/tmp/output.mp4".to_string()), false);
        let (output, video_temp, audio_temp) = get_paths("ignored title", &cli);

        assert_eq!(output, PathBuf::from("/tmp/output.mp4"));
        assert_eq!(video_temp, PathBuf::from("/tmp/output-video.tmp"));
        assert_eq!(audio_temp, PathBuf::from("/tmp/output-audio.tmp"));
    }

    #[test]
    fn test_relative_file_with_extension() {
        let cli = create_cli_demo(Some("videos/output.mp4".to_string()), false);
        let (output, video_temp, audio_temp) = get_paths("ignored", &cli);

        let current_dir = get_current_dir();
        let expected_output_dir = current_dir.join("videos");
        assert_eq!(output, expected_output_dir.join("output.mp4"));
        assert_eq!(video_temp, expected_output_dir.join("output-video.tmp"));
        assert_eq!(audio_temp, expected_output_dir.join("output-audio.tmp"));
    }
}
