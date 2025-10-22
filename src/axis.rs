/// An axis in 2D space, either vertical or horizontal. (X or Y)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
    /// Get the opposite axis of the current axis (Vertical <-> Horizontal)
    pub fn opposite(&self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
        }
    }
}

pub trait ValueForAxis<T> {
    fn value_for_axis(&self, axis: Axis) -> T;
}

pub trait SizeForAxis<T> {
    fn size_for_axis(&self, axis: Axis) -> T;
}
