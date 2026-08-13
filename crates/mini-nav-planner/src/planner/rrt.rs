use crate::{
    errors::PlannerError,
    planner::{Planner, Trajectory},
};
use tracing::info;

pub struct RRT {}

impl RRT {
    pub fn new() -> anyhow::Result<Self, PlannerError> {
        // Ok(Self {})
        Err(PlannerError::PathPlanError(
            "Failed to plan the path".into(),
        ))
    }
}

impl Planner for RRT {
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Trajectory {
        let dist = ((goal[0] - start[0]).powi(2) + (goal[1] - start[1]).powi(2)).sqrt();
        info!("distance to reach goal: {}", dist);
        Vec::new()
    }
}
