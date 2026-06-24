use crate::bar::cli_bar::CliProgressBar;
use crate::downloader::download::download_file;
use crate::extract::bilibili::get_download_url;
use crate::extract::bvid::get_bvid_from_url;
use crate::processer::process::{ProcessError, ProcessOption, process};
use crate::util::path::get_paths;
use crate::util::temp::{add_temp_file, drop_temp_file};
use clap::Parser;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::sync::CancellationToken;

mod bar;
mod cli;
mod downloader;
mod extract;
mod macros;
mod processer;
mod util;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let cancel = CancellationToken::new();

    let cancel_handler = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            println!("清理喵...");
            cancel_handler.cancel();
        }
    });

    tokio::select! {
        _ = cancel.cancelled() => {
            exit_ok();
        }
        res = app(cli, cancel.clone()) => {
            match res {
                Ok(_) => exit_ok(),
                Err(e) => exit_err(e),
            }
        }
    }
}

async fn app(cli: cli::Cli, cancel: CancellationToken) -> Result<(), Box<dyn Error>> {
    let bvid = get_bvid_from_url(&cli.url).ok_or_else(|| "获取bvid失败喵".to_string())?;

    let (video_url, audio_url, mut title, headers) = get_download_url(&bvid).await?;
    if title.is_empty() {
        title = "downloaded_video".to_string();
    }

    println!("\n视频标题: {}", title);
    println!("BVID: {}\n", bvid);

    let (output_file, video_temp_file, audio_temp_file) = get_paths(&title, &cli);
    println!("准备下载到: {}", output_file.display());

    if !cli.confirm_all {
        if output_file.try_exists().unwrap_or(false) {
            println!("注意喵！目标文件已存在 继续下载将覆盖同名文件");
        }

        println!("按回车继续喵...");
        wait_for_enter(cancel.clone()).await?;
    }

    println!("开始下载喵...");

    let audio_task = download_file(
        &audio_url,
        &audio_temp_file,
        &headers,
        CliProgressBar::new("下载音频喵..."),
        cancel.clone(),
    );

    if !cli.only_audio {
        add_temp_file(&video_temp_file);
        add_temp_file(&audio_temp_file);

        let video_task = download_file(
            &video_url,
            &video_temp_file,
            &headers,
            CliProgressBar::new("下载视频喵..."),
            cancel.clone(),
        );

        let (v_res, a_res) = tokio::join!(video_task, audio_task);
        v_res?;
        a_res?;
        println!("下载完成喵...");
    } else {
        add_temp_file(&audio_temp_file);
        audio_task.await?;
        println!("下载完成喵...");
    }

    let process_message = if cli.only_audio {
        "音频处理喵..."
    } else {
        "视频合并喵..."
    };
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

    process(
        process_option,
        CliProgressBar::new(process_message),
        cancel.clone(),
    )
    .await
    .map_err(|e| {
        if matches!(e, ProcessError::Cancelled()) && output_file.exists() {
            let _ = std::fs::remove_file(&output_file);
        }
        e
    })?;

    println!("\n下到了: {}", output_file.display());
    Ok(())
}

async fn wait_for_enter(cancel: CancellationToken) -> Result<(), Box<dyn Error>> {
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    tokio::select! {
        _ = stdin.read_line(&mut line) => Ok(()),
        _ = cancel.cancelled() => Err("取消操作喵".into()),
    }
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
