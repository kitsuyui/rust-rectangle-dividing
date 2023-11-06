use crate::axis::{Axis, ValueForAxis};
use crate::component::Component;
use crate::rotate::Rotate;
use crate::vector::Vector;
/// A point in 2D space
#[derive(Debug, PartialEq, Clone)]
pub struct Point<T>
where
    T: Copy,
{
    x: T,
    y: T,
}

impl<T> ValueForAxis<T> for Point<T>
where
    T: Copy,
{
    fn value_for_axis(&self, axis: &Axis) -> T {
        match axis {
            Axis::Vertical => self.x,
            Axis::Horizontal => self.y,
        }
    }
}

impl<T> Component<T> for Point<T>
where
    T: Copy,
{
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

/// A point in 2D space constructor
impl<T> Point<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// A point in 2D space with default values. in many cases, this is (0, 0)
impl<T> std::default::Default for Point<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

/// Vector from point A to point B
impl<T> std::ops::Sub<Point<T>> for Point<T>
where
    T: Copy + std::ops::Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Rotate a point by 90 degrees
impl<T> Rotate for Point<T>
where
    T: Copy,
{
    fn rotate(&self) -> Self {
        Point {
            x: self.y,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Point::new(2, 2);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_default() {
        let result = Point::<i32>::default();
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn test_value_for_axis() {
        let result = Point::new(2, 3);
        assert_eq!(result.value_for_axis(&Axis::Vertical), 2);
        assert_eq!(result.value_for_axis(&Axis::Horizontal), 3);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(2, 2);
        let b = Point::new(1, 1);
        let result = a - b;
        assert_eq!(result.x(), 1);
        assert_eq!(result.y(), 1);
    }

    #[test]
    fn test_rotate() {
        let result = Point::new(2, 3).rotate();
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 2);
    }
}
