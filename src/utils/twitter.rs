use std::{io::{self, Write}, process};

const DEFAULT_PATH: &str = "yt-dlp"; 

pub struct TwitterVideoOptions {
    pub url: String,
    pub output_path: String,
    pub authentication: String,
    pub log_level: String,
}

pub struct TwitterDownloader {
    pub path: String,
}

impl TwitterDownloader {
    pub fn new() -> Self {
        TwitterDownloader {
            path: DEFAULT_PATH.to_owned()
        }
    }

    pub fn download(&self, options: &TwitterVideoOptions) {
        let cmd = process::Command::new("yt-dlp")
            .args([
                format!("--{}", &options.log_level),
                format!("--{}", &options.authentication),
                format!("-o {}", &options.output_path),
                options.url.clone()
            ])
            .output()
            .expect("Failed to download video");

        io::stdout().write_all(&cmd.stdout).unwrap();
        io::stderr().write_all(&cmd.stderr).unwrap();
    }
}
