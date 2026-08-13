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

use hiroz::Builder;
use hiroz::Result;
use hiroz::context::ZContext;
use hiroz::msg::NativeCdrSerdes;
use hiroz::node::ZNode;
use hiroz::pubsub::ZSub;
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
    #[expect(dead_code)]
    odom_topic: ZSub<RosOdometry, Sample, NativeCdrSerdes<RosOdometry>>,
    planner: MyPlanner<T>,
}

impl<T: ConstructiblePlanner> PlannerNode<T> {
    pub fn new(config: &PlannerConfig, ctx: &ZContext) -> Result<Self> {
        let child = T::new()?;
        let planner = MyPlanner::new(child);
        let node = ctx.create_node(&config.node_name).build()?;
        let odom_sub = node.create_sub::<RosOdometry>(&config.odom_topic).build()?;
        let map_sub = node.create_sub(&config.map_topic).build()?;
        Ok(Self {
            node,
            map_topic: map_sub,
            odom_topic: odom_sub,
            planner,
        })
    }

    pub fn run(&self) {
        let start = &[5.0, 5.0];
        let goal = &[10.0, 10.0];
        let traj = self.planner.plan(start, goal);
        println!("running ti");
        info!("traj: {:?}", traj);
    }
}
