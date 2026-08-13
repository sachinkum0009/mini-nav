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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = DMatrix::<f64>::zeros(10, 10);
    println!("{:?}", grid);
    let new_grid = grid.add_scalar(3.0);
    println!("{:?}", new_grid);
    println!("value is {}", new_grid[(13, 2)]);
    Ok(())
}
