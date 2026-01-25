use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("获取视频bvid视频失败")]
    FailedToGetBvid,

    #[error("初始化网络客户端失败")]
    FailedToInitClient,

    #[error("页面响应为空")]
    EmptyHTMLResponse,

    #[error("解析失败")]
    FailedToParse,

    #[error("JSON解析失败")]
    FailedToParseJSON(#[from] serde_json::Error),

    #[error("请求失败")]
    FailedToRequest(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum GetVideoInfoError {
    #[error("")]
    Error,
}

#[derive(Debug, Error)]
pub enum GetVideoCollectionError {
    #[error("")]
    Error,
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("网络请求失败: {0}")]
    Request(#[from] reqwest::Error),

    #[error("请求头构造失败: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("数据解析失败: 缺少字段 {0}")]
    ParseError(&'static str),
}
