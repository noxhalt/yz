use async_std::task;
use regex::Regex;
use percent_encoding::percent_decode_str;
use serde_json::value::Value;
use serde::{Serialize, Deserialize};

use crate::util;

const VIDEO_INFO_URL: &str = "https://www.youtube.com/get_video_info";

pub struct VideoInfo {
    video_id: String,
    data: Value,
}

impl VideoInfo {
    pub fn from_id(video_id: &str) -> Result<VideoInfo, &str> {
        task::block_on(
            async {
                let mut data =
                    surf::get(format!("{}?{}={}", VIDEO_INFO_URL, "video_id", video_id)).await
                        .expect("Failed to access youtube.");
                let data = data.body_string().await
                    .expect("Failed to get video_info.");
                let pattern = Regex::new(r"player_response=(?P<json>[^&]+)")
                    .expect("Failed to build Regex pattern.");

                let data = pattern.captures(&data)
                    .ok_or("Failed to capture data from player_response.")?
                    .name("json")
                    .ok_or("Failed to get data from player_response.")?
                    .as_str();
                let data = percent_decode_str(data)
                    .decode_utf8()
                    .map_err(|_| "Failed to percent decode json.")?
                    .to_string();

                Ok(VideoInfo {
                    video_id: video_id.to_string(),
                    data: serde_json::from_str::<Value>(&data)
                        .map_err(|_| "Failed to parse video_info.")?,
                })
            }
        )
    }

    pub fn get_json(&self) -> Value {
        self.data.clone()
    }

    pub fn get_adaptive(&self) -> Result<Vec<Adaptive>, &str> {
        let data = self.data
            .get("streamingData").ok_or("Failed to get streaming data.")?
            .get("adaptiveFormats").ok_or("Failed to get adaptive formats.")?;

        println!("{}", data);

        let data: Vec<Adaptive> = serde_json::from_value(data.to_owned()).ok().ok_or("Failed to parse adaptive formats.")?;
        Ok(data)
    }

    pub fn get_best(&self) -> Result<(Option<Adaptive>, Option<Adaptive>), &str> {
        let streams = self.get_adaptive()?;

        let videos = streams.clone();
        let mut videos: Vec<Adaptive> = videos
            .iter()
            .filter(|item| item.height.is_some())
            .cloned()
            .collect();
        videos.sort_by(util::video_ordering);

        let audios = streams.clone();
        let mut audios: Vec<Adaptive> = audios
            .iter()
            .filter(|item| item.audio_sample_rate.is_some())
            .cloned()
            .collect();
        audios.sort_by(util::audio_ordering);

        Ok((audios.last().cloned(), videos.last().cloned()))
    }

    pub fn get_title(&self) -> Result<&str, &str> {
        let title = self.data
            .get("videoDetails")
            .ok_or("Failed to get videoDetails")?
            .get("title")
            .ok_or("Failed to get videoDetails.title")?;
        let title = title.as_str()
            .ok_or("Failed to convert videoDetail.title to string")?;
        Ok(title)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Adaptive {
    pub itag: i64,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub bitrate: Option<i64>,
    pub average_bitrate: Option<i64>,
    pub last_modified: Option<String>,
    pub content_length: Option<String>,
    pub quality: Option<String>,
    pub projection_type: Option<String>,
    pub approx_duration_ms: Option<String>,

    pub audio_quality: Option<String>,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<i64>,
    pub loudness_db: Option<f64>,

    pub width: Option<i64>,
    pub height: Option<i64>,
    pub fps: Option<i64>,
    pub quality_label: Option<String>,
    pub color_info: Option<Value>,
    pub high_replication: Option<bool>,
}
