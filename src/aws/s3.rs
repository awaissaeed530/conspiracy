use aws_config::meta::region::RegionProviderChain;

pub async fn client() -> aws_sdk_s3::Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    aws_sdk_s3::Client::new(&config)
}
