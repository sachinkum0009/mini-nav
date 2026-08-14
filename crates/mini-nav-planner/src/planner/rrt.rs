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

use crate::errors::PlannerError;
use crate::planner::{ConstructiblePlanner, Planner, Point, Trajectory};
use tracing::info;

/// RRT
///
/// Rapid-exploring Random Tree
pub struct RRT {
    max_iter: u32,
}

impl RRT {
    pub fn new(max_iter: u32) -> Result<Self, PlannerError> {
        // Ok(Self { max_iter })
        Err(PlannerError::NotImplemented(
            "RRT Algo is not implmemented".into(),
        ))
    }
}

impl ConstructiblePlanner for RRT {
    fn new(max_iter: u32) -> anyhow::Result<Self> {
        Ok(Self::new(max_iter)?)
    }
}

impl Planner for RRT {
    fn plan(&self, start: &Point, goal: &Point) -> Trajectory {
        let dist = ((goal[0] - start[0]).powi(2) + (goal[1] - start[1]).powi(2)).sqrt();
        info!("distance to reach goal: {}", dist);
        Vec::new()
    }
}
