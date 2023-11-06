use crate::axis::{Axis, SizeForAxis};
use crate::dividing::VerticalDividingHelper;
use crate::rotate::QuarterRotation;
/// rectangle in 2D space with a width and height

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle<T>
where
    T: Copy,
{
    width: T,
    height: T,
}

impl<T> SizeForAxis<T> for Rectangle<T>
where
    T: Copy,
{
    fn size_for_axis(&self, axis: Axis) -> T {
        match axis {
            Axis::Vertical => self.width,
            Axis::Horizontal => self.height,
        }
    }
}

pub trait RectangleSize<T>
where
    T: Copy,
{
    fn width(&self) -> T;
    fn height(&self) -> T;
}

/// A rectangle in 2D space with a width and height
impl<T> RectangleSize<T> for Rectangle<T>
where
    T: Copy,
{
    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}

/// Area of an axis aligned rectangle
pub trait Area<T> {
    fn area(&self) -> T;
}

impl<T> Area<T> for Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

/// A rectangle in 2D space constructor
impl<T> Rectangle<T>
where
    T: Copy,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// Rotate a rectangle by 90 degrees
impl<T> QuarterRotation for Rectangle<T>
where
    T: Copy,
{
    fn rotate_clockwise(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}

impl<T> VerticalDividingHelper<T> for Rectangle<T>
where
    T: Copy + std::ops::Sub<Output = T>,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical_helper(&self, x: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(x, self.height),
            Self::new(self.width - x, self.height),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;
    use crate::dividing::Dividing;

    use super::*;

    #[test]
    fn test_new() {
        let result = Rectangle::new(2, 3);
        assert_eq!(result.width, 2);
        assert_eq!(result.height, 3);
    }

    #[test]
    fn test_identity() {
        // identity: a rectangle is equal to itself
        let rect = Rectangle::new(2, 3);
        assert_rect_eq(&rect, &rect);
    }

    #[test]
    fn test_rotate() {
        assert_rotate_twice_is_same_as_original(&Rectangle::new(2, 3));
    }

    #[test]
    fn test_area() {
        let result = Rectangle::new(2, 3).area();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_divide_vertical() {
        let (rect_a, rect_b) = Rectangle::new(4, 2).divide_vertical(1);
        assert_rect_eq(&rect_a, &Rectangle::new(1, 2));
        assert_rect_eq(&rect_b, &Rectangle::new(3, 2));
    }

    #[test]
    fn test_divide_horizontal() {
        let (rect_a, rect_b) = Rectangle::new(2, 4).divide_horizontal(1);
        assert_rect_eq(&rect_a, &Rectangle::new(2, 1));
        assert_rect_eq(&rect_b, &Rectangle::new(2, 3));
    }

    #[test]
    fn test_divide() {
        let (rect_a, rect_b) = Rectangle::new(4, 2).divide(1, Axis::Vertical);
        assert_rect_eq(&rect_a, &Rectangle::new(1, 2));
        assert_rect_eq(&rect_b, &Rectangle::new(3, 2));

        let (rect_a, rect_b) = Rectangle::new(2, 4).divide(1, Axis::Horizontal);
        assert_rect_eq(&rect_a, &Rectangle::new(2, 1));
        assert_rect_eq(&rect_b, &Rectangle::new(2, 3));
    }

    #[test]
    fn test_divide_nth() {
        let rect = Rectangle::new(6, 2);
        let divided = rect.divide_by_values_and_axis(&vec![1, 2], Axis::Vertical);
        assert_rect_eq(&divided[0], &Rectangle::new(1, 2));
        assert_rect_eq(&divided[1], &Rectangle::new(2, 2));
        assert_rect_eq(&divided[2], &Rectangle::new(3, 2));

        let rect = Rectangle::new(2, 6);
        let divided = rect.divide_by_values_and_axis(&vec![3, 2], Axis::Horizontal);
        assert_rect_eq(&divided[0], &Rectangle::new(2, 3));
        assert_rect_eq(&divided[1], &Rectangle::new(2, 2));
        assert_rect_eq(&divided[2], &Rectangle::new(2, 1));
    }

    #[test]
    fn test_divide_by_weights() {
        let rect = Rectangle::new(6, 2);
        // values
        let divided1 = rect.divide_by_values_and_axis(&vec![1, 2], Axis::Vertical);

        let rect = Rectangle::new(6, 2);
        let divided2 = rect.divide_by_weights_and_axis(&vec![2, 4, 6], Axis::Vertical);
        assert_eq!(divided1, divided2);
    }

    /// Helper function to assert that two rectangles are equal
    fn assert_rect_eq(rect1: &Rectangle<i32>, rect2: &Rectangle<i32>) {
        assert_rect_has_same_component_is_equal(rect1, rect2);
    }

    /// Assert that two rectangles have the same width and height are equal
    fn assert_rect_has_same_component_is_equal<T>(rect1: &Rectangle<T>, rect2: &Rectangle<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug,
    {
        assert_eq!(rect1.width, rect2.width);
        assert_eq!(rect1.height, rect2.height);
        assert_eq!(rect1, rect2);
    }

    /// Rotate a rectangle twice is the same as the original
    fn assert_rotate_twice_is_same_as_original<T>(rect: &Rectangle<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug,
    {
        let rotated_twice = rect.rotate_clockwise().rotate_clockwise();
        assert_rect_has_same_component_is_equal(rect, &rotated_twice);
        assert_eq!(rotated_twice, *rect);
    }
}
