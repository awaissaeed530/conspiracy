use aws_config::{meta::region::RegionProviderChain, SdkConfig};

pub async fn get_config() -> SdkConfig {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("ap-south-1");

    aws_config::from_env()
        .region(region_provider)
        .load()
        .await
}
