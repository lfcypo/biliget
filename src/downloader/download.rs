use http::HeaderMap;
use std::path::PathBuf;

pub fn download_file(url: &String, dest_file: &PathBuf, headers: &HeaderMap) {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).headers(headers.clone()).send().unwrap();
    let mut file = std::fs::File::create(dest_file).unwrap();
    std::io::copy(&mut response, &mut file).unwrap();
}
