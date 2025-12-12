use crate::clip::ffmpeg::merge_video;
use crate::downloader::download::download_file;
use crate::extract::bilibili::get_download_url;
use crate::extract::bvid::get_bvid_from_url;
use crate::util::temp::{add_temp_file, drop_temp_file};
use std::env;
use std::io;

mod clip;
mod downloader;
mod extract;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("没有给我地址, 你在干什么喵！");
        return;
    }

    let url = &args[1];

    let bvid = match get_bvid_from_url(url) {
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

    let download_dir = match env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            println!("不知道下到哪喵！");
            return;
        }
    };

    let video_temp_file = download_dir.join(format!("{title}-video.tmp"));
    let audio_temp_file = download_dir.join(format!("{title}-audio.tmp"));
    let output_file = download_dir.join(format!("{title}.mp4"));

    println!("准备下到: {}", output_file.display());

    println!("按回车继续喵...");
    io::stdin().read_line(&mut String::new()).unwrap();

    println!();

    println!("下视频喵...");
    if let Err(e) = download_file(&video_url, &video_temp_file, &headers) {
        eprintln!("{}", e);
        return;
    };
    add_temp_file(&video_temp_file);
    println!("下完视频喵...");

    println!("下音频喵...");
    if let Err(e) = download_file(&audio_url, &audio_temp_file, &headers) {
        eprintln!("{}", e);
        return;
    };
    add_temp_file(&audio_temp_file);
    println!("下完音频喵...");

    println!("下完合并喵...");
    if let Err(e) = merge_video(&video_temp_file, &audio_temp_file, &output_file) {
        eprintln!("{}", e);
        return;
    };
    println!("合并完了喵...");

    drop_temp_file();
    println!("清理喵...");

    println!("搞定喵!");

    println!();

    println!("下到了: {}", output_file.display());
    println!("拜拜喵")
}
