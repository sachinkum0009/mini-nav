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

pub mod gmapping;
pub mod graph_pose;
pub mod hector_slam;

/// # Mapping Trait
pub trait Mapping {
    /// Update the map based on sensor data and odometry.
    fn update(&mut self, scan_data: &[f32], odom_pose: &[f32; 3]);

    /// Get the current occupancy grid data (-1 unknown, 0..100 occupied).
    fn get_grid(&self) -> anyhow::Result<Vec<i8>>;

    /// Get the occupancy grid dimensions (rows, cols).
    fn get_grid_dimensions(&self) -> (u32, u32);

    /// Get the current robot pose.
    fn get_robot_pose(&self) -> [f32; 3];

    /// Save the map
    fn save_map(&self, file_name: &str) -> bool;
}

pub use gmapping::GMapping;
pub use graph_pose::GraphPose;
pub use hector_slam::HectorSlam;
