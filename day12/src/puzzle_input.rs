use crate::region::Region;
use crate::shape::Shape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleInput {
    pub shapes: Vec<Shape>,
    pub regions: Vec<Region>,
}
