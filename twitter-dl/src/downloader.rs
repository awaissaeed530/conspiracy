use futures_lite::{io, AsyncBufReadExt, StreamExt};

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

    pub async fn download(&self, options: &TwitterVideoOptions) {
        let mut cmd = self.build_command(&options)
            .stdout(async_process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut lines = io::BufReader::new(cmd.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }
    }

    fn build_command(&self, options: &TwitterVideoOptions) -> async_process::Command {
        let mut cmd = async_process::Command::new(&DEFAULT_PATH);

        if let Some(verbose) = options.verbose {
            if verbose {
                cmd.arg("--verbose");
            }
        }

        if let Some(output_path) = options.output_path.clone() {
            cmd.arg("-o").arg(output_path);
        }

        cmd.arg(format!("--cookies-from-browser={}", &options.cookies_browser));
        cmd.arg(options.url.clone());
        cmd 
    }
}
