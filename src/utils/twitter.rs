use std::{io::{self, BufRead}, process};

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
        let mut cmd = process::Command::new("yt-dlp")
            .args([
                format!("--{}", &options.log_level),
                format!("--{}", &options.authentication),
                "-o".to_owned(),
                options.output_path.clone(),
                options.url.clone()
            ])
            .stdout(process::Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = io::BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("Read: {:?}", line);
            }
        }

        cmd.wait().unwrap();
    }
}
