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

use hiroz::msg::NativeCdrSerdes;
use hiroz::node::ZNode;
use hiroz::pubsub::ZSub;
use hiroz_msgs::nav_msgs::OccupancyGrid as RosOccupancyGrid;
use hiroz_msgs::nav_msgs::Odometry as RosOdometry;
use mini_nav_planner::planner::Planner;
use mini_nav_planner::planner::{AStar, MyPlanner};
use tracing::info;
use zenoh::sample::Sample;

pub struct PlannerNode<T: Planner> {
    planner: MyPlanner<T>,
}

impl<T: Planner> PlannerNode<T> {
    pub fn new(planner_impl: T) -> Self {
        Self {
            planner: MyPlanner::new(planner_impl),
        }
    }

    pub fn run(&self) {
        let start = &[5.0, 5.0];
        let goal = &[10.0, 10.0];
        let traj = self.planner.plan(start, goal);
        println!("running ti");
        info!("traj: {:?}", traj);
    }
}
