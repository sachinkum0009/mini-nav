use std::sync::Mutex;

use nalgebra::DMatrix;
use rayon::prelude::*;

use crate::mapping::Mapping;

pub struct HectorSlam {
    _grid_size: u32,
    _resolution: f32,
    grid: Mutex<DMatrix<f32>>,
    robot_pose: [f32; 3],
}

impl HectorSlam {
    pub fn new(grid_size: u32, resolution: f32) -> Self {
        let grid = Mutex::new(DMatrix::zeros(grid_size as usize, grid_size as usize));
        Self {
            _grid_size: grid_size,
            _resolution: resolution,
            grid,
            robot_pose: [0.0, 0.0, 0.0],
        }
    }
}

impl Mapping for HectorSlam {
    fn update(&mut self, _scan_data: &[f32], odom_pose: &[f32; 3]) {
        self.robot_pose = *odom_pose;
        let _my_grid = DMatrix::<f64>::zeros(self._grid_size as usize, self._grid_size as usize);
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
        (self._grid_size, self._grid_size)
    }

    fn get_robot_pose(&self) -> [f32; 3] {
        self.robot_pose
    }
}
