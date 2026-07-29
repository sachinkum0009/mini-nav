use std::sync::Mutex;

use nalgebra::DMatrix;
use rayon::prelude::*;

use crate::mapping::{Mapping, Pose};

/// GMapping
pub struct GMapping {
    grid_size: u32,
    resolution: f32,
    grid: Mutex<DMatrix<f32>>,
    robot_pose: Pose,
}

impl GMapping {
    pub fn new(grid_size: u32, resolution: f32) -> Self {
        let grid = Mutex::new(DMatrix::zeros(grid_size as usize, grid_size as usize));
        Self {
            grid_size: grid_size,
            resolution: resolution,
            grid: grid,
            robot_pose: Pose::default(),
        }
    }

    /// Convert world coordinates to grid indices.
    fn world_to_grid(&self, x: f32, y: f32) -> (usize, usize) {
        let grid_x = ((x / self.resolution) + (self.grid_size as f32 / 2.0)) as usize;
        let grid_y = ((y / self.resolution) + (self.grid_size as f32 / 2.0)) as usize;
        (grid_x, grid_y)
    }
}

impl Mapping for GMapping {
    fn update(&mut self, scan_ranges: &[f32], scan_angles: &[f32], odom_pose: &Pose) {
        // Update the robot pose
        self.robot_pose = odom_pose.clone();

        // Process each laser scan point in parallel
        scan_ranges
            .par_iter() // Parallel iterator
            .zip(scan_angles.par_iter())
            .for_each(|(&range, &angle)| {
                if range.is_nan() || range.is_infinite() {
                    return; // Skip invalid measurements
                }

                // Convert polar to Cartesian coordinates relative to the robot
                let x_robot = range * angle.cos();
                let y_robot = range * angle.sin();

                // Transform to world coordinates
                let x_world = self.robot_pose.x + x_robot * self.robot_pose.theta.cos()
                    - y_robot * self.robot_pose.theta.sin();
                let y_world = self.robot_pose.y
                    + x_robot * self.robot_pose.theta.sin()
                    + y_robot * self.robot_pose.theta.cos();

                // Convert world coordinates to grid indices
                let (grid_x, grid_y) = self.world_to_grid(x_world, y_world);

                // Check if the indices are within bounds
                if grid_x < self.grid_size as usize && grid_y < self.grid_size as usize {
                    // Mark the cell as occupied (1.0)
                    // Note: This is unsafe in parallel contexts! See note below.
                    let mut grid = self.grid.lock().unwrap();
                    grid[(grid_x, grid_y)] = 1.0;
                }
            });
    }

    fn get_grid(&self) -> DMatrix<f32> {
        self.grid.lock().unwrap().clone()
    }

    fn get_robot_pose(&self) -> Pose {
        self.robot_pose.clone()
    }
}
