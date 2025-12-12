use clap::Parser;

#[derive(Parser)]
#[command(about = "简单的B站视频下载工具 可以免登录下载B站高清视频", long_about = None)]
pub struct Cli {
    /// Bilibili 视频地址
    pub url: String,

    /// 仅下载音频
    #[arg(short = 'a', long = "audio", default_value_t = false)]
    pub only_audio: bool,
}
