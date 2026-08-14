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

use std::{fs, time::Duration};

use anyhow::Ok;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannerConfig {
    #[serde(default = "default_node_name")]
    pub node_name: String,
    #[serde(default = "default_map_topic")]
    pub map_topic: String,
    #[serde(default = "default_odom_topic")]
    pub odom_topic: String,
    #[serde(default = "default_goal_topic")]
    pub goal_topic: String,
    #[serde(default = "default_planner_name")]
    pub planner_name: String,
    #[serde(default = "default_timer_callback")]
    pub timer_callback: Duration,
    #[serde(default = "default_max_iter")]
    pub max_iter: u32,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            node_name: "planner_node".to_string(),
            map_topic: "map".to_string(),
            odom_topic: "odom".to_string(),
            goal_topic: "goal".to_string(),
            planner_name: "rrt".to_string(),
            timer_callback: Duration::from_millis(100),
            max_iter: 100,
        }
    }
}

impl PlannerConfig {
    /// Loads a [`PlannerConfig`] from a YAML configuration file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read from `path`.
    /// - The YAML content cannot be parsed into [`PlannerConfig`].
    pub fn from_yaml_file(path: &str) -> anyhow::Result<Self> {
        let yaml_content = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&yaml_content)?;
        Ok(config)
    }
}

// Default functions for serde
fn default_node_name() -> String {
    "planner_node".to_string()
}

fn default_map_topic() -> String {
    "map".to_string()
}

fn default_odom_topic() -> String {
    "odom".to_string()
}

fn default_goal_topic() -> String {
    "goal".to_string()
}

fn default_planner_name() -> String {
    "a_star".to_string()
}

fn default_timer_callback() -> Duration {
    Duration::from_millis(100)
}

fn default_max_iter() -> u32 {
    100
}
