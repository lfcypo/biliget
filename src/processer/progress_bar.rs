use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum FfmpegStatus {
    #[default]
    Continue,
    End,
}

#[derive(Debug, Default, Clone)]
pub struct FfmpegProgress {
    pub status: FfmpegStatus,
    pub frame: u64,
    pub out_time_us: u64,
}

impl FfmpegProgress {
    pub fn from_lines(lines: &[&str]) -> Self {
        let map: HashMap<&str, &str> = lines
            .iter()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                let key = parts.next()?.trim();
                let val = parts.next()?.trim();
                Some((key, val))
            })
            .collect();

        let mut prog = Self::default();

        if let Some(v) = map.get("progress") {
            prog.status = match *v {
                "continue" => FfmpegStatus::Continue,
                "end" => FfmpegStatus::End,
                _ => FfmpegStatus::default(),
            };
        }

        if let Some(v) = map.get("frame") {
            prog.frame = v.parse().unwrap_or(0);
        }

        if let Some(v) = map.get("out_time_us") {
            prog.out_time_us = v.parse().unwrap_or(0);
        }

        prog
    }
}
