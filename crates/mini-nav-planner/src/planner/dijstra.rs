use crate::planner::{Planner, Trajectory};
use tracing::info;

/// Dijstra Algorithm
pub struct Dijstra {
    pub max_iter: i32,
}

impl Dijstra {
    /// Initialize the Dijstra Algorithm
    pub fn new(max_iter: i32) -> Self {
        Self { max_iter }
    }
}

impl Planner for Dijstra {
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Trajectory {
        info!("Planning from start: {:?} to goal: {:?}", start, goal);
        Vec::new()
    }
}
