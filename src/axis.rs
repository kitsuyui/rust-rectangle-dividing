#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
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
