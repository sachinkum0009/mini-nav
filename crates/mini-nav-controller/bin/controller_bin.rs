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

use std::{cell::RefCell, rc::Rc, sync::Arc};

// use mini_nav_controller::controller::pid::PIDController;

use mini_nav_controller::controller::{
    Controller, ControllerFactory, TuningConfig,
    pid::{PidController, PidFactory},
};
use nalgebra::Matrix3;
use tokio::sync::Mutex;

fn run_control_loop<F: ControllerFactory>(factory: F) {
    let config = factory.create_default_config();
    config.display_parameters();

    let controller = factory.create_controller(&config);
    let signal = controller.compute_action(10.0, 12.0);
    println!("Control Signal Output: {}\n", signal);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_control_loop(PidFactory);
    // let mut pid_controller = PIDController::default();
    // pid_controller.print_pid_gains();
    // pid_controller.update_pid(10.0, 3.0, 1.0);
    // pid_controller.step();
    // pid_controller.print_pid_gains();

    // // let pid_controller2 = PIDController::default();
    // let mut pid_controller_box = Box::new(PIDController::default());
    // pid_controller_box.print_pid_gains();
    // pid_controller_box.update_pid(3.0, 2.0, 5.0);
    // pid_controller_box.print_pid_gains();

    // let pid_controller_rc = Rc::new(RefCell::new(PIDController::default()));
    // pid_controller_rc.borrow().print_pid_gains();
    // pid_controller_rc.borrow_mut().update_pid(3.0, 2.0, 5.0);
    // pid_controller_rc.borrow().print_pid_gains();

    // let pid_controller_arc = Arc::new(Mutex::new(PIDController::default()));

    // // Use .lock().unwrap() to acquire the lock and access the data safely
    // pid_controller_arc.try_lock()?.print_pid_gains();
    // pid_controller_arc.try_lock()?.update_pid(3.0, 2.0, 5.0);
    // pid_controller_arc.try_lock()?.print_pid_gains();
    // pid_controller_arc.update_pid(3.0, 2.0, 5.0);
    // pid_controller_arc.print_pid_gains();

    // let mut matrix_a = Matrix3::<f32>::zeros();
    // println!("matrix a: {}", matrix_a);
    // matrix_a.add_scalar_mut(3.5);
    // println!("matrix a: {}", matrix_a);
    // matrix_a = matrix_a * 2.0;
    // println!("matrix a: {}", matrix_a);
    // matrix_a = matrix_a / 3.0;
    // println!("matrix a: {}", matrix_a);
    Ok(())
}
