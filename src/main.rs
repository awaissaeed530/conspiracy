use crate::utils::twitter;

mod utils;

fn main() {
    let options = twitter::TwitterVideoOptions {
        url: "https://twitter.com/iluminatibot/status/1696770527237488863".to_owned(),
        output_path: "video.mp4".to_owned(),
        authentication: "cookies-from-browser=firefox".to_owned(),
        log_level: "verbose".to_owned(),
    };
    let downloader = twitter::TwitterDownloader::new();
    downloader.download(&options);
}
