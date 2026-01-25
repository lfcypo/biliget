use crate::extract::error::ExtractError;
use crate::extract::headers::generate_default_headers;
use crate::extract::result::ExtractResult;
use memchr::memchr_iter;
use reqwest::blocking::Response;

const START_MARK: &[u8; 20] = b"window.__playinfo__=";
const END_MARK: &[u8; 9] = b"</script>";

#[derive(Clone)]
pub struct Extractor {
    /// 待提取的url
    pub url: String,

    /// 提取结果
    pub extract_result: Option<ExtractResult>,

    /// 请求客户端
    pub client: reqwest::blocking::Client,
}

impl Extractor {
    pub fn new(url: &str) -> Result<Self, ExtractError> {
        let client = match reqwest::blocking::Client::builder()
            .default_headers(generate_default_headers())
            .timeout(std::time::Duration::from_secs(10))
            .tcp_nodelay(true)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .gzip(true)
            .build()
        {
            Ok(client) => client,
            Err(_) => return Err(ExtractError::FailedToInitClient),
        };

        Ok(Extractor {
            url: url.to_string(),
            extract_result: None,
            client,
        })
    }

    pub fn extract(&self) -> Result<ExtractResult, ExtractError> {
        if self.extract_result.is_some() {
            // 已经提取过了
            return Ok(self.clone().extract_result.unwrap());
        };

        // 获取主页源码 检测合集
        let resp: Response = self.client.get(&self.url).send()?;
        let html: String = resp.text()?;
        // dbg!(&html);
        let play_info = extract_play_info_from_html(html.as_str())?;

        println!("{:?}", play_info);

        Ok(ExtractResult { videos: None })
    }
}

/// 从页面提取视频合集信息
pub fn extract_play_info_from_html(html: &str) -> Result<String, ExtractError> {
    if html.is_empty() {
        return Err(ExtractError::EmptyHTMLResponse);
    }

    dbg!(html);

    let html_bytes = html.as_bytes();

    // let (start_mark_pos, end_mark_pos) = match aaa(html_bytes) {
    //     Ok(pos) => {
    //         found_times += 1;
    //         if found_times == 2 {
    //             pos
    //         } else {
    //             aaa(html_bytes)?
    //         }
    //     }
    //     Err(e) => return Err(e),
    // };

    let (start_mark_pos, end_mark_pos) = match_json_nth(html_bytes, 1).unwrap();

    let json_start = start_mark_pos + START_MARK.len();
    let json_end = end_mark_pos;
    let json_part = &html[json_start..json_end];

    dbg!(json_start);
    dbg!(json_end);
    println!("{:#}", json_part);

    // let mut brace_count = 0;
    // let mut json_end_idx = None;
    // for (idx, c) in json_part.chars().enumerate() {
    //     match c {
    //         '{' => brace_count += 1,
    //         '}' => {
    //             brace_count -= 1;
    //             if brace_count == 0 {
    //                 json_end_idx = Some(idx + 1);
    //                 break;
    //             }
    //         }
    //         ' ' | '\n' | '\r' | '\t' if brace_count == 0 => continue,
    //         _ if brace_count == 0 => return Err(ExtractError::FailedToParse),
    //         _ => {}
    //     }
    // }
    //
    // let json_end_idx = json_end_idx.ok_or(ExtractError::FailedToParse)?;
    // let json_str = &json_part[..json_end_idx];
    //
    // dbg!(json_str);

    // let json = serde_json::from_str(json_part)?;
    Ok(String::new())
}

fn match_json_nth(html_bytes: &[u8], n: usize) -> Result<(usize, usize), ExtractError> {
    if n == 0 {
        return Err(ExtractError::FailedToParse);
    }

    let mut current_pos = 0;
    let mut match_count = 0;

    while match_count < n {
        let start_mark_pos = match memchr_iter(START_MARK[0], &html_bytes[current_pos..])
            .find(|&i| {
                let absolute_pos = i + current_pos;
                absolute_pos + START_MARK.len() <= html_bytes.len()
                    && &html_bytes[absolute_pos..absolute_pos + START_MARK.len()] == START_MARK
            }) {
            None => return Err(ExtractError::FailedToParse),
            Some(relative_pos) => relative_pos + current_pos,
        };

        let end_mark_pos = match memchr_iter(END_MARK[0], &html_bytes[start_mark_pos..])
            .find(|&i| {
                let absolute_pos = i + start_mark_pos;
                absolute_pos + END_MARK.len() <= html_bytes.len()
                    && &html_bytes[absolute_pos..absolute_pos + END_MARK.len()] == END_MARK
            }) {
            None => return Err(ExtractError::FailedToParse),
            Some(relative_pos) => relative_pos + start_mark_pos,
        };

        match_count += 1;

        if match_count == n {
            return Ok((start_mark_pos, end_mark_pos));
        }

        current_pos = end_mark_pos + END_MARK.len();
    }

    Err(ExtractError::FailedToParse)
}
