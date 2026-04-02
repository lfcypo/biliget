use regex::Regex;
use std::sync::LazyLock;

static BVID_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bBV\w{10}\b").unwrap());

pub fn get_bvid_from_url(url: &str) -> Option<String> {
    BVID_REGEX.find(url).map(|mat| mat.as_str().to_string())
}
