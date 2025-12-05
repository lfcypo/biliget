use crate::util::http::default_http_headers;
use http::HeaderMap;
use reqwest;
use serde_json::Value;

const BILIBILI_DOWNLOAD_URL_API_URL: &str =
    "https://api.bilibili.com/x/player/playurl?qn=80&fnval=4048&fourk=1&try_look=1";
const BILIBILI_INFO_API_URL: &str = "https://api.bilibili.com/x/player/pagelist";

fn get_info(bvid: &String) -> Result<(i64, String), &'static str> {
    let referrer = format!("https://www.bilibili.com/video/{bvid}");
    let mut headers = default_http_headers();
    headers.insert("referer", referrer.parse().unwrap());
    headers.insert("origin", "https://www.bilibili.com".parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let json: Value = client
        .get(BILIBILI_INFO_API_URL)
        .headers(headers)
        .query(&[("bvid", bvid)])
        .send()
        .unwrap()
        .json()
        .unwrap();

    Ok((
        json["data"][0]["cid"].as_i64().unwrap(),
        json["data"][0]["part"].as_str().unwrap().to_string(),
    ))
}

pub fn get_download_url(
    bvid: &String,
) -> Result<(String, String, String, HeaderMap), &'static str> {
    let (cid, title) = get_info(bvid)?;

    let referrer = format!("https://www.bilibili.com/video/{bvid}");
    let mut headers = default_http_headers();
    headers.insert("referer", referrer.parse().unwrap());
    headers.insert("origin", "https://www.bilibili.com".parse().unwrap());

    let client = reqwest::blocking::Client::new();
    let json: Value = client
        .get(BILIBILI_DOWNLOAD_URL_API_URL)
        .headers(headers.clone())
        .query(&[("bvid", bvid)])
        .query(&[("cid", cid)])
        .send()
        .unwrap()
        .json()
        .unwrap();

    Ok((
        json["data"]["dash"]["video"][0]["baseUrl"]
            .as_str()
            .unwrap()
            .to_string(),
        json["data"]["dash"]["audio"][0]["baseUrl"]
            .as_str()
            .unwrap()
            .to_string(),
        title,
        headers,
    ))
}
