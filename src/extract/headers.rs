use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{ORIGIN as HED_ORIGIN, REFERER as HED_REFERER, USER_AGENT as HED_USER_AGENT};

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
const BASE_URL: &str = "https://www.bilibili.com";

pub fn generate_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(HED_USER_AGENT, HeaderValue::from_static(USER_AGENT));
    headers.insert(HED_ORIGIN, HeaderValue::from_static(BASE_URL));
    headers.insert(HED_REFERER, HeaderValue::from_static(BASE_URL));
    headers
}

pub fn generate_default_headers_with_referer(referer: &str) -> HeaderMap {
    let mut headers = generate_default_headers();
    headers.insert(HED_REFERER, HeaderValue::from_str(referer).unwrap());
    headers
}
