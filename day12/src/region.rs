#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub shape_counts: Vec<usize>,
}

impl Region {
    pub fn new(width: usize, height: usize, shape_counts: Vec<usize>) -> Self {
        Region {
            width,
            height,
            shape_counts,
        }
    }
}
