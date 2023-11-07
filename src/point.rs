use num_traits::{Float, Num, NumAssignOps, NumOps};

use crate::axis::{Axis, ValueForAxis};
use crate::component::Component;
use crate::rotate::QuarterRotation;
use crate::vector::Vector;
/// A point in 2D space
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    x: T,
    y: T,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Edge {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

impl<T> Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps + Float,
{
    pub fn round(&self, edge: Edge) -> Self {
        match edge {
            Edge::LeftTop => Self {
                x: self.x.floor(),
                y: self.y.floor(),
            },
            Edge::RightTop => Self {
                x: self.x.ceil(),
                y: self.y.floor(),
            },
            Edge::LeftBottom => Self {
                x: self.x.floor(),
                y: self.y.ceil(),
            },
            Edge::RightBottom => Self {
                x: self.x.ceil(),
                y: self.y.ceil(),
            },
        }
    }
}

impl<T> ValueForAxis<T> for Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn value_for_axis(&self, axis: Axis) -> T {
        match axis {
            Axis::Vertical => self.x,
            Axis::Horizontal => self.y,
        }
    }
}

impl<T> Component<T> for Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
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
    T: Copy + Num + NumAssignOps + NumOps,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// A point in 2D space with default values. in many cases, this is (0, 0)
impl<T> std::default::Default for Point<T>
where
    T: Default + Copy + Num + NumAssignOps + NumOps,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

/// Vector from point A to point B
impl<T> std::ops::Sub<Point<T>> for Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// Rotate a point by 90 degrees
impl<T> QuarterRotation for Point<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn rotate_clockwise(&self) -> Self {
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
        assert_point_eq(&result, &Point::new(0, 0));
    }

    #[test]
    fn test_value_for_axis() {
        let result = Point::new(2, 3);
        assert_eq!(result.value_for_axis(Axis::Vertical), 2);
        assert_eq!(result.value_for_axis(Axis::Horizontal), 3);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(2, 2);
        let b = Point::new(1, 1);
        assert_ne!(a, b);
        let result = a - b;
        assert_eq!(result, Vector::new(1, 1));
    }

    #[test]
    fn test_rotate() {
        let result = Point::new(2, 3).rotate_clockwise();
        assert_point_eq(&result, &Point::new(3, 2));
    }

    /// Helper function to assert that two points are equal
    fn assert_point_eq<T>(p1: &Point<T>, p2: &Point<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug + Num + NumAssignOps + NumOps,
    {
        assert_point_has_same_component_is_equal(p1, p2);
    }

    /// Assert that two points have the same component values
    fn assert_point_has_same_component_is_equal<T>(p1: &Point<T>, p2: &Point<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug + Num + NumAssignOps + NumOps,
    {
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
        assert_eq!(p1, p2);
    }
}
