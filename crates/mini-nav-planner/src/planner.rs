pub mod a_star;
pub mod dijstra;
pub mod hybrid_a_star;
pub mod rrt;

/// Planner Trait
pub trait Planner {
    /// Plans the trajectory from start to goal position
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Vec<f32>;
}

pub use a_star::AStar;
pub use dijstra::Dijstra;
pub use hybrid_a_star::HybridAStar;
pub use rrt::RRT;
