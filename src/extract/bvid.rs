use regex::Regex;

lazy_static::lazy_static! {
    static ref BVID_REGEX: Regex = Regex::new(r"\bBV\w{10}\b").unwrap();
}

pub fn get_bvid_from_url(url: &str) -> Option<String> {
    BVID_REGEX.find(url).map(|mat| mat.as_str().to_string())
}
