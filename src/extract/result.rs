#[derive(Clone)]
pub struct Video {
    /// 视频标题
    pub title: String,
    /// 视频bvid
    pub bvid: String,

    /// 视频的视频地址
    pub video_url: String,
    /// 视频的音频地址
    pub audio_url: String,
}

#[derive(Clone)]
pub struct ExtractResult {
    /// 提取到的合集视频
    /// 如果为单个视频则Vec中有且只有一个
    pub videos: Option<Vec<Video>>,
}
