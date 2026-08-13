pub mod a_star;
pub mod dijstra;
pub mod hybrid_a_star;
pub mod rrt;

// types
pub type Point = [f32; 2];
pub type Trajectory = Vec<Point>;

/// Planner Trait
pub trait Planner {
    /// Plans the trajectory from start to goal position
    fn plan(&self, start: &Point, goal: &Point) -> Trajectory;
}

pub use a_star::AStar;
pub use dijstra::Dijstra;
pub use hybrid_a_star::HybridAStar;
pub use rrt::RRT;
