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

use nalgebra::DMatrix;

use crate::controller::{Controller, ControllerFactory, TuningConfig};

// #[derive(Debug, Default)]
// pub struct PIDController {
//     p: f64,
//     i: f64,
//     d: f64,
// }

// // impl Default for PIDController {
// //     fn default() -> Self {
// //         Self {
// //             p: 0.1,
// //             i: 0.1,
// //             d: 0.1,
// //         }
// //     }
// // }

// impl PIDController {
//     /// Steps forward to the PID controller
//     pub fn step(&self) -> f64 {
//         10.0
//     }

//     /// Update the PID gains
//     pub fn update_pid(&mut self, p: f64, i: f64, d: f64) {
//         self.p = p;
//         self.i = i;
//         self.d = d;
//     }

//     pub fn print_pid_gains(&self) {
//         println!("p: {}, i: {}, d: {}", self.p, self.i, self.d);
//     }
// }

// use crate::{Controller, ControllerFactory, TuningConfig};

// 1. Concrete Config
pub struct PidConfig {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
}

impl TuningConfig for PidConfig {
    fn display_parameters(&self) {
        println!(
            "PID Gains -> Kp: {}, Ki: {}, Kd: {}",
            self.kp, self.ki, self.kd
        );
    }
}

// 2. Concrete Controller
pub struct PidController {
    kp: f64,
    ki: f64,
    kd: f64,
}

impl Controller for PidController {
    fn compute_action(&self, current_state: f64, target_state: f64) -> f64 {
        let error = target_state - current_state;
        // Simplified proportional action for the example
        error * self.kp + error * self.ki + error * self.kd
    }
}

// 3. Concrete Factory
pub struct PidFactory;

impl ControllerFactory for PidFactory {
    type C = PidController;
    type T = PidConfig;

    fn create_controller(&self, config: &Self::T) -> Self::C {
        PidController {
            kp: config.kp,
            ki: config.ki,
            kd: config.kd,
        }
    }

    fn create_default_config(&self) -> Self::T {
        PidConfig {
            kp: 1.0,
            ki: 0.1,
            kd: 0.05,
        }
    }
}
