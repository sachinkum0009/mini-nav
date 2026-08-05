use hiroz::Result;
use std::{fs, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GmappingConfig {
    #[serde(default = "default_node_name")]
    pub node_name: String,
    #[serde(default = "default_laser_topic")]
    pub laser_topic: String,
    #[serde(default = "default_odom_topic")]
    pub odom_topic: String,
    #[serde(default = "default_map_topic")]
    pub map_topic: String,
    #[serde(default = "default_map_name")]
    pub map_name: String,
    #[serde(default = "default_grid_size")]
    pub grid_size: u32,
    #[serde(default = "default_resolution")]
    pub resolution: f32,
    #[serde(default = "default_timer_callback")]
    pub timer_callback: Duration,
}

impl Default for GmappingConfig {
    fn default() -> Self {
        Self {
            node_name: "gmapping_node".to_string(),
            laser_topic: "scan".to_string(),
            odom_topic: "odom".to_string(),
            map_topic: "map".to_string(),
            map_name: "map".to_string(),
            grid_size: 1000,
            resolution: 0.05,
            timer_callback: Duration::from_millis(100),
        }
    }
}

impl GmappingConfig {
    pub fn from_yaml_file(path: &str) -> Result<Self> {
        let yaml_content = fs::read_to_string(path)?;
        let config: GmappingConfig = serde_yaml::from_str(&yaml_content)?;
        Ok(config)
    }
}

// Default functions for serde
fn default_node_name() -> String {
    "gmapping_node".to_string()
}

fn default_laser_topic() -> String {
    "scan".to_string()
}

fn default_odom_topic() -> String {
    "odom".to_string()
}

fn default_map_topic() -> String {
    "map".to_string()
}

fn default_map_name() -> String {
    "map".to_string()
}

fn default_grid_size() -> u32 {
    100
}

fn default_resolution() -> f32 {
    0.1
}

fn default_timer_callback() -> Duration {
    Duration::from_millis(100)
}
