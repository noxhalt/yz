use crate::youtube::Adaptive;
use std::cmp::Ordering;
use std::str::FromStr;

pub fn video_ordering(a: &Adaptive, b: &Adaptive) -> Ordering {
    match a.height.unwrap_or(0).cmp(&b.height.unwrap_or(0)) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.average_bitrate.unwrap_or(0).cmp(&b.average_bitrate.unwrap_or(0)),
        Ordering::Greater => Ordering::Greater,
    }
}

pub fn audio_ordering(a: &Adaptive, b: &Adaptive) -> Ordering {
    let a_sample = i64::from_str(a.audio_sample_rate.as_ref().unwrap()).unwrap_or(0);
    let b_sample = i64::from_str(b.audio_sample_rate.as_ref().unwrap()).unwrap_or(0);
    match a_sample.cmp(&b_sample) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.average_bitrate.unwrap_or(0).cmp(&b.average_bitrate.unwrap_or(0)),
        Ordering::Greater => Ordering::Greater,
    }
}
