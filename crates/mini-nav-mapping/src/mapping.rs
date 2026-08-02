pub mod gmapping;
pub mod graph_pose;
pub mod hector_slam;

/// # Mapping Trait
pub trait Mapping {
    /// Update the map based on sensor data and odometry.
    fn update(&mut self, scan_data: &[f32], odom_pose: &[f32; 3]);

    /// Get the current occupancy grid data (-1 unknown, 0..100 occupied).
    fn get_grid(&self) -> Vec<i8>;

    /// Get the occupancy grid dimensions (rows, cols).
    fn get_grid_dimensions(&self) -> (u32, u32);

    /// Get the current robot pose.
    fn get_robot_pose(&self) -> [f32; 3];
}

pub use gmapping::GMapping;
pub use graph_pose::GraphPose;
pub use hector_slam::HectorSlam;
