use std::time::Duration;

use hiroz::{
    Builder, Result,
    context::ZContext,
    msg::NativeCdrSerdes,
    node::ZNode,
    pubsub::{ZPub, ZSub},
};
use hiroz_msgs::nav_msgs::Odometry as RosOdometry;
use hiroz_msgs::sensor_msgs::LaserScan as RosLaserScan;
use hiroz_msgs::{builtin_interfaces::Time, nav_msgs::OccupancyGrid as RosOccupancyGrid};
use mini_nav_mapping::mapping::{GMapping, Mapping};
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

    /// # Convert Quaternion to Euler
    ///
    /// ## Arguments
    /// - quat: [f32; 4] x, y, z, w
    ///
    /// ## Returns
    /// - euler: [f32; 3] roll, pitch, yaw
    fn convert_quaternion_to_euler(_quat: &[f64; 4]) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    /// # Convert LaserScan to cartesian points
    ///
    /// Filters out invalid measurements and returns the scan points as
    /// interleaved (x, y) coordinates relative to the robot.
    fn laserscan_to_cartesian(scan: &RosLaserScan) -> Vec<f32> {
        let mut points = Vec::with_capacity(scan.ranges.len() * 2);
        for (i, &r) in scan.ranges.iter().enumerate() {
            if !r.is_finite() || r < scan.range_min || r > scan.range_max {
                continue;
            }
            let angle = scan.angle_min + i as f32 * scan.angle_increment;
            points.push(r * angle.cos());
            points.push(r * angle.sin());
        }
        points
    }

    /// # Spin the ZNode
    ///
    pub async fn spin(&mut self, duration: &Duration) -> Result<()> {
        let mut msg = RosOccupancyGrid::default();
        msg.header.frame_id = "map".into();
        let (rows, cols) = self.gmapping.get_grid_dimensions();
        msg.info.width = cols;
        msg.info.height = rows;
        msg.info.resolution = 0.1;

        loop {
            let odom_msg = self.odom_topic.async_recv().await?;
            let scan_msg = self.laser_topic.async_recv().await?;
            let scan_data = Self::laserscan_to_cartesian(&scan_msg);
            let euler = Self::convert_quaternion_to_euler(&[
                odom_msg.pose.pose.orientation.x,
                odom_msg.pose.pose.orientation.y,
                odom_msg.pose.pose.orientation.z,
                odom_msg.pose.pose.orientation.w,
            ]);
            let odom_pose = [
                odom_msg.pose.pose.position.x as f32,
                odom_msg.pose.pose.position.y as f32,
                euler[2],
            ];
            self.gmapping.update(&scan_data, &odom_pose);
            msg.data = self.gmapping.get_grid();
            let t = self.node.clock().now().as_unix_nanos();
            msg.header.stamp = Time {
                sec: (t / 1_000_000_000) as i32,
                nanosec: (t % 1_000_000_000) as u32,
            };
            self.publish_map(&msg).await?;
            tokio::time::sleep(duration.clone()).await;
        }
    }

    /// # Publish the Occupancy Grid Map
    ///
    /// ## Arguments
    /// - msg: OccupancyGrid
    pub async fn publish_map(&self, msg: &RosOccupancyGrid) -> Result<()> {
        self.map_topic.async_publish(msg).await?;
        Ok(())
    }
}
