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

/// Controller
pub mod pid;

// Abstract factory for the controllers
pub trait AbstractFactory {
    fn build();
}

// Abstract Products (Traits)
pub trait Controller {
    fn compute_action(&self, current_state: f64, target_state: f64) -> f64;
}

pub trait TuningConfig {
    fn display_parameters(&self);
}

// The Abstract Factory
pub trait ControllerFactory {
    type C: Controller;
    type T: TuningConfig;

    fn create_controller(&self, config: &Self::T) -> Self::C;
    fn create_default_config(&self) -> Self::T;
}
