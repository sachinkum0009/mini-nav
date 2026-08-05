use hiroz::{Builder, Result, context::ZContextBuilder};
use mini_nav::configs::GmappingConfig;
use mini_nav::nav_sub::NavPub;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = ZContextBuilder::default()
        .with_connect_endpoints(["tcp/127.0.0.1:7447"])
        .with_shm_enabled()? // Enable with defaults: 10MB pool, 512B threshold
        .build()?;
    let gmapping_config = GmappingConfig::from_yaml_file("config.yaml")?;
    let mut nav_node = NavPub::new(&gmapping_config, &ctx)?;
    nav_node.spin(&gmapping_config.timer_callback).await?;
    Ok(())
}
