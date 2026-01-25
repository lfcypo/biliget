use crate::extract::error::FetchError;
use crate::util::http::default_http_headers;
use http::HeaderMap;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayInfo {}

const BILIBILI_DOWNLOAD_URL_API_URL: &str =
    "https://api.bilibili.com/x/player/playurl?qn=80&fnval=4048&fourk=1&try_look=1";
const BILIBILI_INFO_API_URL: &str = "https://api.bilibili.com/x/player/pagelist";

fn get_info(client: &reqwest::blocking::Client, bvid: &str) -> Result<(i64, String), FetchError> {
    let json: Value = client
        .get(BILIBILI_INFO_API_URL)
        .query(&[("bvid", bvid)])
        .send()?
        .json()?;

    let cid = if let Some(cid) = json["data"][0]["cid"].as_i64() {
        cid
    } else {
        return Err(FetchError::ParseError("cid"));
    };

    let title = if let Some(title) = json["data"][0]["part"].as_str() {
        title.to_string()
    } else {
        return Err(FetchError::ParseError("title"));
    };

    Ok((cid, title))
}

pub fn get_download_url(bvid: &str) -> Result<(String, String, String, HeaderMap), FetchError> {
    let referrer = format!("https://www.bilibili.com/video/{bvid}");
    let mut headers = default_http_headers();
    headers.insert("referer", referrer.parse()?);
    headers.insert("origin", "https://www.bilibili.com".parse()?);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers.clone())
        .build()?;

    let (cid, title) = get_info(&client, bvid)?;

    let json: Value = client
        .get(BILIBILI_DOWNLOAD_URL_API_URL)
        .query(&[("bvid", bvid)])
        .query(&[("cid", cid)])
        .send()?
        .json()?;

    let video_url = if let Some(video_url) = json["data"]["dash"]["video"][0]["baseUrl"].as_str() {
        video_url.to_string()
    } else {
        return Err(FetchError::ParseError("video_url"));
    };

    let audio_url = if let Some(audio_url) = json["data"]["dash"]["audio"][0]["baseUrl"].as_str() {
        audio_url.to_string()
    } else {
        return Err(FetchError::ParseError("audio_url"));
    };

    Ok((video_url, audio_url, title, headers))
}
