use std::sync::Mutex;

use nalgebra::DMatrix;

use crate::mapping::{Mapping, Pose};

pub struct HectorSlam {
    grid_size: u32,
    resolution: f32,
    grid: Mutex<DMatrix<f32>>,
    robot_pose: Pose,
}

impl HectorSlam {
    pub fn new(grid_size: u32, resolution: f32) -> Self {
        let grid = Mutex::new(DMatrix::zeros(grid_size as usize, grid_size as usize));
        Self {
            grid_size,
            resolution,
            grid,
            robot_pose: Pose::default(),
        }
    }
}

impl Mapping for HectorSlam {
    fn update(&mut self, scan_range: &[f32], scan_angles: &[f32], odom_pose: &Pose) {
        let grid_size = 10;
        let my_grid = DMatrix::<f64>::zeros(grid_size as usize, grid_size as usize);
    }

    fn get_grid(&self) -> DMatrix<f32> {
        self.grid.lock().unwrap().clone()
    }

    fn get_robot_pose(&self) -> Pose {
        self.robot_pose.clone()
    }
}
