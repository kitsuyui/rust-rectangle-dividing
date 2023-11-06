pub enum Axis {
    Vertical,
    Horizontal,
}

pub trait ValueForAxis<T> {
    fn value_for_axis(&self, axis: &Axis) -> T;
}

pub trait SizeForAxis<T> {
    fn size_for_axis(&self, axis: &Axis) -> T;
}
