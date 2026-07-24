pub trait Mapping {
    fn map(&self);
}

/// GraphSLAM
struct GraphSLAM {}

impl Mapping for GraphSLAM {
    fn map(&self) {}
}
