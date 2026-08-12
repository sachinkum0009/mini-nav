use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlannerError {
    #[error("Failed to plan path")]
    PathPlanError(String),
}
