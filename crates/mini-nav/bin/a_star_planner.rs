use mini_nav_planner::planner::{AStar, Planner};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let start = &[10.0, 20.0];
    let goal = &[10.0, 20.0];
    let a_star = AStar::new()?;
    let traj = a_star.plan(start, goal);
    info!("traj: {:?}", traj);
    Ok(())
}
