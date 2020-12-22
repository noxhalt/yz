mod youtube;
mod util;

use youtube::{VideoInfo, Adaptive};
use clap::{App, load_yaml, ArgMatches};
use std::process::{Command, ExitStatus};

fn download(data: (Option<Adaptive>, Option<Adaptive>), matches: ArgMatches) -> Result<ExitStatus, &'static str> {
    let seek = matches.value_of("seek");
    let time = matches.value_of("time");
    let output = matches.value_of("output").unwrap();

    // build args
    let mut args = vec![];
    let audio = data.0;
    let video = data.1;

    if let Some(audio) = audio.as_ref() {
        if let Some(seek) = seek {
            args.push("-ss");
            args.push(&seek);
        }
        args.push("-i");
        args.push(audio.url.as_ref().ok_or("Failed to get audio download url")?);
    }

    if let Some(video) = video.as_ref() {
        if let Some(seek) = seek {
            args.push("-ss");
            args.push(&seek);
        }
        args.push("-i");
        args.push(video.url.as_ref().ok_or("Failed to get video download url")?);
    }

    if let Some(time) = time {
        args.push("-t");
        args.push(&time);
    }

    args.push(output);
    args.push("-y");

    let mut child = Command::new("ffmpeg")
        .args(args)
        .spawn()
        .expect("Failed to run process");

    // child.stdout;
    match child.wait() {
        Ok(status) => Ok(status),
        Err(_) => Err("Failed to get exit status")
    }
}

fn main() {
    let yaml = load_yaml!("args.yaml");
    let matches = App::from(yaml).get_matches();
    let video_id = matches.value_of("video").unwrap();
    let video_info = VideoInfo::from_id(video_id).unwrap();
    let video_title = video_info.get_title().unwrap();
    let best = video_info.get_best().unwrap();

    println!("Downloading: {}", video_title);
    download(best, matches).unwrap();
}
