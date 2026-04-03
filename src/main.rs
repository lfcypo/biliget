use crate::downloader::download::download_file;
use crate::extract::bilibili::get_download_url;
use crate::extract::bvid::get_bvid_from_url;
use crate::processer::process::{ProcessError, ProcessOption, process};
use crate::progress::cli_bar::CliProgressBar;
use crate::util::path::get_paths;
use crate::util::temp::{add_temp_file, drop_temp_file};
use clap::Parser;
use std::error::Error;
use tokio::fs::remove_file;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

mod cli;
mod downloader;
mod extract;
mod macros;
mod processer;
mod progress;
mod util;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    let cancel = CancellationToken::new();

    let cancel_clone = cancel.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("退出程序喵...");
        cancel_clone.cancel();
    });

    let bvid = match get_bvid_from_url(&cli.url) {
        Some(bvid) => bvid,
        _ => {
            println!("获取bvid失败喵");
            return exit_err(Box::from("获取bvid失败".to_string()));
        }
    };
    let (video_url, audio_url, mut title, headers) = match get_download_url(&bvid).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return exit_err(Box::from(e));
        }
    };
    title = if title.is_empty() {
        "downloaded_video".to_string()
    } else {
        title
    };

    println!();
    println!("视频标题: {}", title);
    println!("BVID: {}", bvid);
    println!();

    let (output_file, video_temp_file, audio_temp_file) = get_paths(&title, &cli);
    println!("准备下到: {}", output_file.display());

    if output_file.try_exists().unwrap_or(false) {
        println!("注意喵！目标文件已存在 继续下载将覆盖同名文件");
    }

    println!("按回车继续喵...");
    let mut stdin = BufReader::new(io::stdin());
    let mut input = String::new();
    tokio::select! {
        _ = stdin.read_line(&mut input) => {},
        () = cancel.cancelled() => {
            return exit_ok();
        }
    }

    not_cancelled_println!(cancel);

    not_cancelled_println!(cancel, "开始下载喵...");

    let download_video_task = if !cli.only_audio {
        Some(async {
            add_temp_file(&video_temp_file);
            download_file(
                &video_url,
                &video_temp_file,
                &headers,
                CliProgressBar::new("下载视频"),
                cancel.clone(),
            )
            .await
        })
    } else {
        None
    };

    let download_audio_task = async {
        add_temp_file(&audio_temp_file);
        download_file(
            &audio_url,
            &audio_temp_file,
            &headers,
            CliProgressBar::new("下载音频"),
            cancel.clone(),
        )
        .await
    };

    match download_video_task {
        Some(video_task) => {
            let (video_result, audio_result) = tokio::join!(video_task, download_audio_task);

            if let Err(e) = video_result {
                eprintln!("视频下载失败");
                return exit_err(Box::from(e));
            } else {
                not_cancelled_println!(cancel, "下完视频喵...");
            }

            if let Err(e) = audio_result {
                eprintln!("音频下载失败");
                return exit_err(Box::from(e));
            }
            not_cancelled_println!(cancel, "下完音频喵...");
        }
        None => {
            if let Err(e) = download_audio_task.await {
                eprintln!("音频下载失败");
                return exit_err(Box::from(e));
            }
            not_cancelled_println!(cancel, "下完音频喵...");
        }
    }

    let process_option = ProcessOption {
        video_file: if cli.only_audio {
            None
        } else {
            Some(&video_temp_file)
        },
        audio_file: Some(&audio_temp_file),
        output_file: &output_file,
        only_audio: cli.only_audio,
    };
    if let Err(e) = process(process_option, cancel.clone()).await {
        match e {
            ProcessError::Cancelled() => {
                if !output_file.try_exists().unwrap_or(false) {
                    exit_ok();
                }
                if let Err(e) = remove_file(&output_file).await {
                    exit_err(Box::from(e))
                }
            }
            e => {
                exit_err(Box::from(e));
            }
        }
    } else {
        not_cancelled_println!(cancel, "后处理结束喵...");
    }

    not_cancelled_println!(cancel);

    not_cancelled_println!(cancel, "下到了: {}", output_file.display());

    exit_ok()
}

fn exit_ok() {
    drop_temp_file();
    println!("拜拜喵");
    std::process::exit(0);
}

fn exit_err(err: Box<dyn Error>) {
    drop_temp_file();
    eprintln!("{}", err);
    std::process::exit(1);
}
