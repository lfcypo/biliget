use crate::downloader::download::download_file;
use crate::extract::bilibili::get_download_url;
use crate::extract::bvid::get_bvid_from_url;
use crate::processer::process::{ProcessOption, process};
use crate::util::path::get_paths;
use crate::util::temp::{add_temp_file, drop_temp_file};
use clap::Parser;
use std::io;

mod cli;
mod downloader;
mod extract;
mod processer;
mod util;

fn main() {
    let cli = cli::Cli::parse();

    ctrlc::set_handler(move || {
        drop_temp_file();
        std::process::exit(0);
    })
    .expect("");

    let bvid = match get_bvid_from_url(&cli.url) {
        Some(bvid) => bvid,
        _ => {
            println!("获取bvid失败喵");
            return;
        }
    };
    let (video_url, audio_url, mut title, headers) = match get_download_url(&bvid) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return;
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

    println!("按回车继续喵...");
    io::stdin().read_line(&mut String::new()).unwrap();

    println!();

    if !cli.only_audio {
        println!("下视频喵...");
        add_temp_file(&video_temp_file);
        if let Err(e) = download_file(&video_url, &video_temp_file, &headers) {
            eprintln!("{}", e);
            return;
        };
        println!("下完视频喵...");
    }

    println!("下音频喵...");
    add_temp_file(&audio_temp_file);
    if let Err(e) = download_file(&audio_url, &audio_temp_file, &headers) {
        eprintln!("{}", e);
        return;
    };
    println!("下完音频喵...");

    println!("后处理喵...");
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
    if let Err(e) = process(process_option) {
        eprintln!("{}", e);
        return;
    };
    println!("后处理结束喵...");

    drop_temp_file();
    println!("清理喵...");

    println!("搞定喵!");

    println!();

    println!("下到了: {}", output_file.display());
    println!("拜拜喵")
}
