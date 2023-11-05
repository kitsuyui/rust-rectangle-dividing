pub enum Direction {
    Vertical,
    Horizontal,
}

pub trait ValueForDirection<T> {
    fn value_for_direction(&self, direction: &Direction) -> T;
}

pub trait SizeForDirection<T> {
    fn size_for_direction(&self, direction: &Direction) -> T;
}
