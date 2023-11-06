use crate::axis::{Axis, ValueForAxis};
use crate::component::Component;

/// A simple 2D vector
#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T>
where
    T: Copy,
{
    x: T,
    y: T,
}

/// A simple 2D vector constructor
impl<T> Vector<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Component<T> for Vector<T>
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

impl<T> ValueForAxis<T> for Vector<T>
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

/// A simple 2D vector with default values. in many cases, this is (0, 0)
impl<T> std::default::Default for Vector<T>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

/// Add vector A to vector B
impl<T> std::ops::Add<Vector<T>> for Vector<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/// Subtract vector B from vector A
impl<T> std::ops::Sub<Vector<T>> for Vector<T>
where
    T: Copy + std::ops::Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Vector::new(2, 2);
        assert_eq!(result.x(), 2);
        assert_eq!(result.y(), 2);
    }

    #[test]
    fn test_default() {
        let result = Vector::<i32>::default();
        assert_eq!(result.x(), 0);
        assert_eq!(result.y(), 0);
    }

    #[test]
    fn test_add() {
        let a = Vector::new(2, 2);
        let b = Vector::new(1, 1);
        let result = a + b;
        assert_eq!(result.x(), 3);
        assert_eq!(result.y(), 3);
    }
}
