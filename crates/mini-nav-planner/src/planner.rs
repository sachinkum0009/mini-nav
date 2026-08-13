// Copyright 2026 Sachin Kumar.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod a_star;
pub mod dijstra;
pub mod hybrid_a_star;
pub mod rrt;

// types
pub type Point = [f64; 2];
pub type Trajectory = Vec<Point>;

/// Planner Trait
pub trait Planner {
    /// Plans the trajectory from start to goal position
    fn plan(&self, start: &Point, goal: &Point) -> Trajectory;
}

pub trait ConstructiblePlanner: Planner + Sized {
    fn new() -> anyhow::Result<Self>;
}

pub struct MyPlanner<T: Planner> {
    child: T,
}

impl<T: Planner> MyPlanner<T> {
    pub fn new(child: T) -> Self {
        MyPlanner { child }
    }
    pub fn plan(&self, start: &Point, goal: &Point) -> Trajectory {
        self.child.plan(start, goal)
    }
}

pub use a_star::AStar;
pub use dijstra::Dijstra;
pub use hybrid_a_star::HybridAStar;
pub use rrt::RRT;
