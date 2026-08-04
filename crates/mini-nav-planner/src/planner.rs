pub mod a_star;

/// Planner Trait
pub trait Planner {
    /// Plans the trajectory from start to goal position
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Vec<f32>;
}
