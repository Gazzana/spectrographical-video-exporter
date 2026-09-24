# Spectrographical Video Exporter

SVE is a sonogram creation software built on rust, inspired by [EAnalysis](https://github.com/pierrecouprie/eanalysis).

## Overview

Processing of spectrograms and batch video renderization often are slow and tediuos tasks. SVE wraps tools like [SoX](https://sourceforge.net/projects/sox/) and [FFmpeg](https://ffmpeg.org/) in a clean graphical interface for enhanced user experience

## Features

* **Batch processing:** give SVE a reference folder and it will make all sonograms by itself.

* **Format Bulletproofing:** Any format will be temporarily transformed to `wav` for processing reasons

* **Synchronized Playhead**

## Building
``` bash
git clone https://github.com/Gazzana/spectrographical-video-exporter.git

cd spectrographical-video-exporter
```

### Running in developing mode

``` bash
cargo install dioxus-cli

dx serve
```

### Build

``` bash
cargo build --release
```
## Dependencies
User must have these installed in order to use SVE
* [FFmpeg and FFprobe](https://ffmpeg.org/)
* [SoX](https://sourceforge.net/projects/sox/)
