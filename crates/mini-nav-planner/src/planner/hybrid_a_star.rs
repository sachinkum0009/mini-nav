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

// TODO: Implement Hybrid A Star Algorithm for path planning

use tracing::info;

use crate::{
    errors::PlannerError,
    planner::{Planner, Trajectory},
};

/// # Hybrid A Star
///
/// Path planning algorithm which considers the kinematics of the robot while planning the path, to deliver the trajectory which is drivable by robot.
pub struct HybridAStar {}

impl HybridAStar {
    /// Initialize the HybridAStar
    pub fn new() -> anyhow::Result<Self, PlannerError> {
        // Ok(Self {})
        Err(PlannerError::PathPlanError(
            "Failed to plan the path".into(),
        ))
    }
}

impl Planner for HybridAStar {
    fn plan(&self, start: &[f32; 2], goal: &[f32; 2]) -> Trajectory {
        info!("Planning from {:?} to {:?}", start, goal);
        Vec::new()
    }
}
