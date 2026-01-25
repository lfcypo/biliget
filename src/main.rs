use crate::downloader::download::download_file;
use crate::processer::process::{ProcessOption, process};
use crate::util::temp::{add_temp_file, drop_temp_file};
use clap::Parser;
use std::env;
use std::io;

mod cli;
mod downloader;
mod extract;
mod processer;
mod util;

fn main() {
    let ext = extract::extract::Extractor::new("https://www.bilibili.com/video/BV1E4sHzrEfY/?spm_id_from=333.1391.0.0&vd_source=99d9bc5e383382f5726b131115c24938").unwrap();
    let _ = ext.extract();

    // let cli = cli::Cli::parse();
    //
    // let bvid = match get_bvid_from_url(&cli.url) {
    //     Some(bvid) => bvid,
    //     _ => {
    //         println!("获取bvid失败喵");
    //         return;
    //     }
    // };
    // let (video_url, audio_url, mut title, headers) = match get_download_url(&bvid) {
    //     Ok(data) => data,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         return;
    //     }
    // };
    // title = if title.is_empty() {
    //     "downloaded_video".to_string()
    // } else {
    //     title
    // };
    //
    // println!();
    // println!("视频标题: {}", title);
    // println!("BVID: {}", bvid);
    // println!();
    //
    // let download_dir = match env::current_dir() {
    //     Ok(path) => path,
    //     Err(_) => {
    //         println!("不知道下到哪喵！");
    //         return;
    //     }
    // };
    //
    // let video_temp_file = download_dir.join(format!("{title}-video.tmp"));
    // let audio_temp_file = download_dir.join(format!("{title}-audio.tmp"));
    // let output_file = if cli.only_audio {
    //     download_dir.join(format!("{title}.wav"))
    // } else {
    //     download_dir.join(format!("{title}.mp4"))
    // };
    //
    // println!("准备下到: {}", output_file.display());
    //
    // println!("按回车继续喵...");
    // io::stdin().read_line(&mut String::new()).unwrap();
    //
    // println!();
    //
    // if !cli.only_audio {
    //     println!("下视频喵...");
    //     if let Err(e) = download_file(&video_url, &video_temp_file, &headers) {
    //         eprintln!("{}", e);
    //         return;
    //     };
    //     add_temp_file(&video_temp_file);
    //     println!("下完视频喵...");
    // }
    //
    // println!("下音频喵...");
    // if let Err(e) = download_file(&audio_url, &audio_temp_file, &headers) {
    //     eprintln!("{}", e);
    //     return;
    // };
    // add_temp_file(&audio_temp_file);
    // println!("下完音频喵...");
    //
    // println!("后处理喵...");
    // let process_option = ProcessOption {
    //     video_file: if cli.only_audio {
    //         None
    //     } else {
    //         Some(&video_temp_file)
    //     },
    //     audio_file: Some(&audio_temp_file),
    //     output_file: &output_file,
    //     only_audio: cli.only_audio,
    // };
    // if let Err(e) = process(process_option) {
    //     eprintln!("{}", e);
    //     return;
    // };
    // println!("后处理结束喵...");
    //
    // drop_temp_file();
    // println!("清理喵...");
    //
    // println!("搞定喵!");
    //
    // println!();
    //
    // println!("下到了: {}", output_file.display());
    // println!("拜拜喵")
}
