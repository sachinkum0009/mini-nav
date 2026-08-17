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

use std::collections::BinaryHeap;

use crate::planner::{ConstructiblePlanner, Planner, Point, Trajectory};
use tracing::info;

struct Node {
    point: Point,
    rank: i32,
}

/// Dijstra Algorithm
pub struct Dijstra {
    pub max_iter: u32,
}

impl Dijstra {
    /// Initialize the Dijstra Algorithm
    pub fn new(max_iter: u32) -> Self {
        Self { max_iter }
    }
}

impl ConstructiblePlanner for Dijstra {
    fn new(max_iter: u32) -> anyhow::Result<Self> {
        Ok(Self::new(max_iter))
    }
}

impl Planner for Dijstra {
    fn plan(&self, start: &Point, goal: &Point) -> Trajectory {
        let node = Node {
            point: *start,
            rank: 0,
        };
        info!("Planning from start: {:?} to goal: {:?}", start, goal);
        Vec::new()
    }
}
