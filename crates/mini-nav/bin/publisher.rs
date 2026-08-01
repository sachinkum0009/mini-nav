use std::time::Duration;

use hiroz::{Builder, Result, context::ZContextBuilder};
use hiroz_msgs::nav_msgs::OccupancyGrid;
use mini_nav::configs::GmappingConfig;
use mini_nav::nav_sub::NavPub;
use mini_nav_mapping::mapping::{GMapping, Mapping};

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = ZContextBuilder::default()
        .with_connect_endpoints(["tcp/127.0.0.1:7447"])
        .build()?;
    let gmapping_config = GmappingConfig::default();
    let nav_node = NavPub::new(&gmapping_config, &ctx)?;
    let gmapping = GMapping::new(100, 0.1);
    let grid = gmapping.get_grid();
    let mut occupancy_grid = OccupancyGrid::default();
    occupancy_grid.header.frame_id = "map".into();
    occupancy_grid.info.width = grid.ncols() as u32;
    occupancy_grid.info.height = grid.nrows() as u32;
    occupancy_grid.info.resolution = 0.1;
    occupancy_grid.data = grid
        .iter()
        .map(|&v| if v < 0.0 { -1 } else { (v * 100.0) as i8 })
        .collect();
    nav_node.spin(&gmapping_config.timer_callback).await?;
    Ok(())
}
