use std::fs;

use aws_sdk_s3::{primitives::ByteStream, types::{CompletedPart, CompletedMultipartUpload}};

use crate::{aws::s3, utils::twitter};

mod utils;
mod aws;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    /*
    let options = twitter::TwitterVideoOptions {
        url: "https://twitter.com/iluminatibot/status/1696770527237488863".to_owned(),
        output_path: "video.mp4".to_owned(),
        authentication: "cookies-from-browser=firefox".to_owned(),
        log_level: "verbose".to_owned(),
    };
    let downloader = twitter::TwitterDownloader::new();
    downloader.download(&options);
    */

    let contents = fs::read("video.mp4").unwrap();

    let bucket = "conspiracy-dev";
    let key = "video.mp4";
    let client = s3::client().await;

    let upload = client.create_multipart_upload()
        .key(key)
        .bucket(bucket)
        .content_type("video/mp4")
        .send()
        .await
        .unwrap();
    println!("{:?}\n", upload);

    let upload_id = upload.upload_id.unwrap();
    println!("{:?}\n", upload_id);

    let part_upload = client.upload_part()
        .upload_id(&upload_id)
        .part_number(1)
        .key(key)
        .body(ByteStream::from(contents))
        .bucket(bucket)
        .send()
        .await
        .unwrap();
    println!("{:?}\n", part_upload);

    let complete_part = CompletedPart::builder()
        .e_tag(part_upload.e_tag.unwrap_or_default())
        .part_number(1)
        .build();

    let completed_multipart_upload = CompletedMultipartUpload::builder()
        .parts(complete_part)
        .build();

    let complete_upload = client.complete_multipart_upload()
        .upload_id(&upload_id)
        .multipart_upload(completed_multipart_upload)
        .key(key)
        .bucket(bucket)
        .send()
        .await
        .unwrap();
    println!("{:?}\n", complete_upload);

    Ok(())
}
