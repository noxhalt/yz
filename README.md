# yz

The simple youtube downloader.  
This is don't have decrypting cipher stream features.

## How to ues?

Require `ffmpeg`, and add it to `PATH`.

```
# simple
./yz [VIDEO_ID] -o output.mp4

# if you needs range
./yz [VIDEO_ID] -s [seek time] -t [duration] -o output.mp4
```

PS. `-s` and `-t` is ffmpeg style.

## Why?

I need simple youtube downloader, when cliping vtuber's stream archive.  
So, tryed it.

