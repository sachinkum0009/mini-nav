// TODO: Implement Hybrid A Star Algorithm for path planning

use crate::planner::Planner;

/// # Hybrid A Star
///
/// Path planning algorithm which considers the kinematics of the robot while planning the path, to deliver the trajectory which is drivable by robot.
pub struct HybridAStar {}

impl HybridAStar {
    /// Initialize the HybridAStar
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl Planner for HybridAStar {
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Vec<f32> {
        let traj = vec![0.0];
        traj
    }
}
