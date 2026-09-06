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

use std::ops::Mul;

use nalgebra::DMatrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = DMatrix::<f64>::zeros(10, 10);
    let grid_b = DMatrix::<f64>::zeros(10, 10);
    let new_grid_b = grid_b.add_scalar(2.0);
    println!("{:?}", grid);
    let new_grid = grid.add_scalar(3.0);
    let new_grid = new_grid * 6.0;
    println!("{:?}", new_grid);
    println!("value is {}", new_grid.get((3, 3)).unwrap());

    let grid_c = new_grid + new_grid_b;
    println!("Grid C: {:?}", grid_c);

    let my_arr = [1, 2, 3, 4];
    let c = my_arr.get(5);

    let my_vec = vec![1, 2, 3];

    // let my_val = my_vec[5];
    // println!("my val: {}", my_val);
    if let Some(val) = my_vec.get(1) {
        println!("val is {}", val);
    }
    match c {
        Some(val) => {
            println!("val is: {}", val);
        }
        None => {
            println!("not able to find that value");
        }
    }

    // let a = my_arr.get(5);
    if let Some(b) = my_arr.get(3) {
        println!("my arr: {:}", b);
    } else {
        println!("there is no value");
    };
    // match a {
    //     Some(data) => {
    //         println!("val is {:?}", data);
    //     }
    //     None => {
    //         println!("there is no value");
    //     }
    // }
    Ok(())
}
