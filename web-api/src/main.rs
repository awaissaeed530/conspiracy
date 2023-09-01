use std::io;

use twitter_dl::{self, downloader::{TwitterDownloader, TwitterVideoOptions}};
use aws::{config::get_config, storage::StorageClient};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut key = uuid::Uuid::new_v4().to_string();
    key.push_str(".mp4");

    let options = TwitterVideoOptions {
        url: "https://twitter.com/iluminatibot/status/1696770527237488863".to_owned(),
        output_path: Some(key.clone()),
        cookies_browser: "firefox".to_owned(),
        verbose: Some(true)
    };
    TwitterDownloader::new().download(&options).await;
    println!("Video has been downloaded locally");

    println!("Starting upload to S3");
    let config = get_config().await;
    let storage_client = StorageClient::new(&config, "conspiracy-dev");
    storage_client.upload_video(&key).await;
    println!("Video has been uploaded to S3");

    Ok(())
}

