use std::{io::{self, BufRead}, process};

const DEFAULT_PATH: &str = "yt-dlp"; 

pub struct TwitterVideoOptions {
    pub url: String,
    pub output_path: Option<String>,
    pub cookies_browser: String,
    pub verbose: Option<bool>,
}

impl Default for TwitterVideoOptions {
    fn default() -> Self {
        TwitterVideoOptions {
            url: String::new(),
            output_path: None,
            cookies_browser: String::new(),
            verbose: None
        }
    }
}

pub struct TwitterDownloader;

impl TwitterDownloader {
    pub fn new() -> Self {
        TwitterDownloader { }
    }

    pub fn download(&self, options: &TwitterVideoOptions) {
        let mut cmd = self.build_command(&options)
            .stdout(process::Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = io::BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("{:?}", line);
            }
        }

        cmd.wait().unwrap();
    }

    fn build_command(&self, options: &TwitterVideoOptions) -> process::Command {
        let mut cmd = process::Command::new(&DEFAULT_PATH);

        if let Some(verbose) = options.verbose {
            if verbose {
                cmd.arg("--verbose");
            }
        }

        if let Some(output_path) = options.output_path.clone() {
            cmd.arg("-o").arg(output_path);
        }

        cmd.arg(format!("--cookies-from-browser={}", &options.cookies_browser));
        cmd.arg(format!("--cookies-from-browser={}", &options.cookies_browser));
        cmd.arg(options.url.clone());
        cmd 
    }
}
