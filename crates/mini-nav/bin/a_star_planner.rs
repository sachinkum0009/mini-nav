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

use mini_nav::nodes::planner_node::PlannerNode;
use mini_nav_planner::planner::{ConstructiblePlanner, Planner, RRT};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Application level result handling
    tracing_subscriber::fmt::init();
    let start = &[5.0, 5.0];
    let goal = &[10.0, 10.0];
    let a_star = RRT::new(100)?;
    // let a_star2 = PlannerNode::<RRT>::new(config, ctx)
    let traj = a_star.plan(start, goal);
    info!("traj: {:?}", traj);
    Ok(())
}
