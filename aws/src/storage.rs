use std::fs;

use aws_sdk_s3::{primitives::ByteStream, types::{CompletedPart, CompletedMultipartUpload}};

pub struct StorageClient {
   s3_client: aws_sdk_s3::Client, 
   bucket: String,
}

impl StorageClient {
    pub fn new(sdk_config: &aws_config::SdkConfig, bucket: &str) -> Self {
        StorageClient {
            s3_client: aws_sdk_s3::Client::new(&sdk_config),
            bucket: bucket.to_owned(),
        }
    }

    pub async fn upload_video(&self, path: &str) {
        let contents = fs::read(path).expect("Failed to load video");
        self.upload_multipart(vec![contents], path, "video/mp4").await;
    }

    async fn upload_multipart(&self, contents: Vec<Vec<u8>>, path: &str, content_type: &str) {
        let upload = self.s3_client.create_multipart_upload()
            .key(path)
            .bucket(&self.bucket)
            .content_type(content_type)
            .send()
            .await
            .unwrap();

        let upload_id = upload.upload_id.unwrap();
        let mut complete_uploads: Vec<CompletedPart> = Vec::new();

        for (index, chunk) in contents.iter().enumerate() {
            let part_number = (index + 1) as i32;
            let part_upload = self.s3_client.upload_part()
                .upload_id(&upload_id)
                .part_number(part_number)
                .key(path)
                .body(ByteStream::from(chunk.to_owned()))
                .bucket(&self.bucket)
                .send()
                .await
                .unwrap();

            complete_uploads.push(
                CompletedPart::builder()
                .e_tag(part_upload.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build()
            );
        }

        let completed_multipart_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(complete_uploads))
            .build();

        self.s3_client.complete_multipart_upload()
            .upload_id(&upload_id)
            .multipart_upload(completed_multipart_upload)
            .key(path)
            .bucket(&self.bucket)
            .send()
            .await
            .unwrap();
    }
}
