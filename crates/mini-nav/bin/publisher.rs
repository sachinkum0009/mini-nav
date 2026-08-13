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
