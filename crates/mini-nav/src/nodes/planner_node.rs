// Copyright 2026 Sachin Kumar.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::Duration;

use hiroz::Builder;
use hiroz::Result;
use hiroz::context::ZContext;
use hiroz::msg::NativeCdrSerdes;
use hiroz::node::ZNode;
use hiroz::pubsub::ZSub;
use hiroz_msgs::geometry_msgs::PointStamped as RosPointStamped;
use hiroz_msgs::nav_msgs::OccupancyGrid as RosOccupancyGrid;
use hiroz_msgs::nav_msgs::Odometry as RosOdometry;
use mini_nav_planner::planner::ConstructiblePlanner;
use mini_nav_planner::planner::{MyPlanner, Planner};
use tracing::info;
use zenoh::sample::Sample;

use crate::configs::PlannerConfig;

pub struct PlannerNode<T: Planner> {
    #[expect(dead_code)]
    node: ZNode,
    #[expect(dead_code)]
    map_topic: ZSub<RosOccupancyGrid, Sample, NativeCdrSerdes<RosOccupancyGrid>>,
    odom_topic: ZSub<RosOdometry, Sample, NativeCdrSerdes<RosOdometry>>,
    goal_topic: ZSub<RosPointStamped, Sample, NativeCdrSerdes<RosPointStamped>>,
    planner: MyPlanner<T>,
}

impl<T: ConstructiblePlanner> PlannerNode<T> {
    pub fn new(config: &PlannerConfig, ctx: &ZContext) -> Result<Self> {
        let child = T::new(config.max_iter)?;
        let planner = MyPlanner::new(child);
        let node = ctx.create_node(&config.node_name).build()?;
        let odom_sub = node.create_sub::<RosOdometry>(&config.odom_topic).build()?;
        let map_sub = node.create_sub(&config.map_topic).build()?;
        let goal_sub = node.create_sub(&config.goal_topic).build()?;
        Ok(Self {
            node,
            map_topic: map_sub,
            odom_topic: odom_sub,
            goal_topic: goal_sub,
            planner,
        })
    }

    /// Spins the node
    pub async fn spin(&self, duration: &Duration) -> Result<()> {
        loop {
            let goal_msg = self.goal_topic.async_recv().await?;
            let odom_msg = self.odom_topic.async_recv().await?;
            let start = &[odom_msg.pose.pose.position.x, odom_msg.pose.pose.position.y];
            let goal = &[goal_msg.point.x, goal_msg.point.y];
            let traj = self.planner.plan(start, goal);
            info!("traj: {:?}", traj);
            tokio::time::sleep(*duration).await;
        }
    }
}
