pub trait Dividing<T> {
    fn divide_vertical(&self, x: T) -> (Self, Self)
    where
        Self: Sized;
    fn divide_horizontal(&self, y: T) -> (Self, Self)
    where
        Self: Sized;
}
