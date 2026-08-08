// TODO: Implement A Star Algorithm for path planning

use crate::planner::Planner;

/// # A Star
///
/// Path planning algorithm with heuristics
pub struct AStar {}

impl AStar {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl Planner for AStar {
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Vec<f32> {
        let traj = vec![0.0];
        traj
    }
}
