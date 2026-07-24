pub trait Planner {
    fn plan(&self);
}

pub struct AStar {}

impl Planner for AStar {
    fn plan(&self) {}
}
