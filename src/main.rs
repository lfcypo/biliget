use crate::clip::ffmpeg::merge_video;
use crate::downloader::download::download_file;
use crate::extract::bilibili::get_download_url;
use crate::extract::bvid::get_bvid_from_url;
use crate::util::temp::{add_temp_file, drop_temp_file};
use std::env;
use std::io;
use std::path::Path;

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

    let bvid = get_bvid_from_url(url).unwrap();
    let (video_url, audio_url, mut title, headers) = get_download_url(&bvid).unwrap();
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
        Ok(path) => path.display().to_string(),
        Err(_) => {
            println!("不知道下到哪喵！");
            return;
        }
    };
    let download_dir = Path::new(&download_dir);

    let video_temp_file = download_dir.join(format!("{title}-video.tmp"));
    let audio_temp_file = download_dir.join(format!("{title}-audio.tmp"));
    let output_file = download_dir.join(format!("{title}.mp4"));

    println!("准备下到: {}", output_file.display());

    println!("按回车继续喵  ᯠ _ ̫  _ ᯄ  ");
    io::stdin().read_line(&mut String::new()).unwrap();

    println!();

    println!("下视频喵...");
    download_file(&video_url, &video_temp_file, &headers);
    add_temp_file(&video_temp_file);
    println!("下完视频喵...");

    println!("下音频喵...");
    download_file(&audio_url, &audio_temp_file, &headers);
    add_temp_file(&audio_temp_file);
    println!("下完音频喵...");

    println!("下完合并喵...");
    merge_video(&video_temp_file, &audio_temp_file, &output_file);
    println!("合并完了喵...");

    drop_temp_file();
    println!("清理喵...");

    println!("搞定喵!");

    println!();

    println!("下到了: {}", output_file.display());
    println!("拜拜喵")
}
