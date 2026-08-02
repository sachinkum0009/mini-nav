use std::sync::Mutex;

use nalgebra::DMatrix;
use rayon::prelude::*;

use crate::mapping::Mapping;

/// GMapping
pub struct GMapping {
    grid_size: u32,
    resolution: f32,
    grid: Mutex<DMatrix<f32>>,
    robot_pose: [f32; 3],
}

impl GMapping {
    pub fn new(grid_size: u32, resolution: f32) -> Self {
        let grid = Mutex::new(DMatrix::zeros(grid_size as usize, grid_size as usize));
        Self {
            grid_size: grid_size,
            resolution: resolution,
            grid: grid,
            robot_pose: [0.0, 0.0, 0.0],
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
    fn update(&mut self, scan_data: &[f32], odom_pose: &[f32; 3]) {
        // Update the robot pose
        self.robot_pose = *odom_pose;

        // Process each laser scan point in parallel (interleaved x, y pairs)
        scan_data
            .par_chunks(2) // Parallel iterator
            .for_each(|point| {
                let x_robot = point[0];
                let y_robot = point[1];
                if !x_robot.is_finite() || !y_robot.is_finite() {
                    return; // Skip invalid measurements
                }

                // Transform to world coordinates
                let x_world = self.robot_pose[0] + x_robot * self.robot_pose[2].cos()
                    - y_robot * self.robot_pose[2].sin();
                let y_world = self.robot_pose[1]
                    + x_robot * self.robot_pose[2].sin()
                    + y_robot * self.robot_pose[2].cos();

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

    fn get_grid(&self) -> Vec<i8> {
        self.grid
            .lock()
            .unwrap()
            .as_slice()
            .par_iter()
            .map(|&v| if v < 0.0 { -1 } else { (v * 100.0) as i8 })
            .collect()
    }

    fn get_grid_dimensions(&self) -> (u32, u32) {
        (self.grid_size, self.grid_size)
    }

    fn get_robot_pose(&self) -> [f32; 3] {
        self.robot_pose
    }
}
