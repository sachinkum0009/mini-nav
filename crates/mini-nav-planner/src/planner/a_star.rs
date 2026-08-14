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

// TODO: Implement A Star Algorithm for path planning

use crate::{
    errors::PlannerError,
    planner::{ConstructiblePlanner, Planner, Point, Trajectory},
};
use tracing::{debug, error, info, warn};

/// # A Star
///
/// Path planning algorithm with heuristics
pub struct AStar {
    max_iter: u32,
}

impl AStar {
    pub fn new(max_iter: u32) -> Result<Self, PlannerError> {
        // Ok(Self { max_iter })
        Err(PlannerError::NotImplemented(
            "AStart algo is not yet implemented".into(),
        ))
    }
}

impl ConstructiblePlanner for AStar {
    fn new(max_iter: u32) -> anyhow::Result<Self> {
        Ok(Self::new(max_iter)?)
    }
}

impl Planner for AStar {
    fn plan(&self, start: &Point, goal: &Point) -> Trajectory {
        error!("this si an error");
        warn!("this is a warning");
        debug!("a star planner started");
        info!("Planning path from {:?} to {:?}", start, goal);
        Vec::new()
    }
}
