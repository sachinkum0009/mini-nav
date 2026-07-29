use nalgebra::DMatrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = DMatrix::<f64>::zeros(10, 10);
    println!("{:?}", grid);
    let new_grid = grid.add_scalar(3.0);
    println!("{:?}", new_grid);
    println!("value is {}", new_grid[(13, 2)]);
    Ok(())
}
