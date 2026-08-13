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

use hiroz::{Builder, Result, context::ZContextBuilder};
use mini_nav::{configs::PlannerConfig, nodes::planner_node::PlannerNode};
#[allow(unused_imports)]
use mini_nav_planner::planner::{ConstructiblePlanner, HybridAStar};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let ctx = ZContextBuilder::default()
        .with_connect_endpoints(["tcp/127.0.0.1:7447"])
        .with_shm_enabled()?
        .build()?;

    // let planner_config = PlannerConfig::from_yaml_file("config.yaml")?;
    let planner_config = PlannerConfig::default();

    let planner_node = PlannerNode::<HybridAStar>::new(&planner_config, &ctx)?;
    planner_node.run().await?;

    Ok(())
}
