use std::time::Duration;

use hiroz::{
    Builder, Result,
    context::ZContext,
    msg::NativeCdrSerdes,
    node::ZNode,
    pubsub::{ZPub, ZSub},
};
use hiroz_msgs::nav_msgs::OccupancyGrid as RosOccupancyGrid;
use hiroz_msgs::nav_msgs::Odometry as RosOdometry;
use hiroz_msgs::sensor_msgs::LaserScan as RosLaserScan;
use mini_nav_mapping::mapping::GMapping;
use zenoh::sample::Sample;

use crate::configs::GmappingConfig;

/// # Nav Pub
///
/// ## Arguments
/// - node: ZNode
/// - topic:
pub struct NavPub {
    node: ZNode,
    laser_topic: ZSub<RosLaserScan, Sample, NativeCdrSerdes<RosLaserScan>>,
    odom_topic: ZSub<RosOdometry, Sample, NativeCdrSerdes<RosOdometry>>,
    map_topic: ZPub<RosOccupancyGrid, NativeCdrSerdes<RosOccupancyGrid>>,
    gmapping: GMapping,
}

impl NavPub {
    pub fn new(config: &GmappingConfig, ctx: &ZContext) -> Result<Self> {
        let node = ctx.create_node(&config.node_name).build()?;
        let laser_sub = node
            .create_sub::<RosLaserScan>(&config.laser_topic)
            .build()?;
        let odom_sub = node.create_sub::<RosOdometry>(&config.odom_topic).build()?;
        let map_pub = node
            .create_pub::<RosOccupancyGrid>(&config.map_topic)
            .build()?;
        let gmapping = GMapping::new(100, 0.1);
        Ok(Self {
            node,
            laser_topic: laser_sub,
            odom_topic: odom_sub,
            map_topic: map_pub,
            gmapping: gmapping,
        })
    }

    /// # Spin the ZNode
    ///
    pub async fn spin(&self, duration: &Duration) -> Result<()> {
        let mut msg = RosOccupancyGrid::default();
        msg.header.frame_id = "map".into();

        loop {
            self.publish_map(&msg).await?;
            tokio::time::sleep(duration.clone()).await;
        }
    }

    pub async fn publish_map(&self, msg: &RosOccupancyGrid) -> Result<()> {
        self.map_topic.async_publish(msg).await?;
        Ok(())
    }
}
