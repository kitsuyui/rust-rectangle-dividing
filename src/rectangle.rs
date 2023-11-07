use crate::area::Area;
use crate::aspect_ratio::AspectRatio;
use crate::axis::{Axis, SizeForAxis};
use crate::dividing::VerticalDividingHelper;
use crate::rotate::QuarterRotation;
use num_traits::{Float, Num, NumAssignOps, NumOps};
/// rectangle in 2D space with a width and height

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    width: T,
    height: T,
}

impl<T> Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + Float,
{
    pub fn round(&self) -> Self {
        Self {
            width: self.width.round(),
            height: self.height.round(),
        }
    }
}

impl<T> SizeForAxis<T> for Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
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
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn width(&self) -> T;
    fn height(&self) -> T;
}

/// A rectangle in 2D space with a width and height
impl<T> RectangleSize<T> for Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn width(&self) -> T {
        self.width
    }

    fn height(&self) -> T {
        self.height
    }
}

impl<T> Area<T> for Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

/// A rectangle in 2D space constructor
impl<T> Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// Rotate a rectangle by 90 degrees
impl<T> QuarterRotation for Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
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
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical_helper(&self, x: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(x, self.height),
            Self::new(self.width - x, self.height),
        )
    }
}

impl<T> AspectRatio<T> for Rectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn aspect_ratio(&self) -> T {
        self.width / self.height
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
    fn test_aspect_ratio() {
        let result = Rectangle::new(16.0, 9.0).aspect_ratio();
        assert_eq!(result, 1.7777777777777777);
        let result = Rectangle::new(1920.0, 1080.0).aspect_ratio();
        assert_eq!(result, 1.7777777777777777);
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
        let rect = Rectangle::new(6.0, 2.0);
        // values
        let divided1 = rect.divide_by_values_and_axis(&vec![1.0, 2.0], Axis::Vertical);

        let rect = Rectangle::new(6.0, 2.0);
        let divided2 = rect.divide_by_weights_and_axis(&vec![2.0, 4.0, 6.0], Axis::Vertical);
        assert_eq!(divided1, divided2);
    }

    /// Helper function to assert that two rectangles are equal
    fn assert_rect_eq(rect1: &Rectangle<i32>, rect2: &Rectangle<i32>) {
        assert_rect_has_same_component_is_equal(rect1, rect2);
    }

    /// Assert that two rectangles have the same width and height are equal
    fn assert_rect_has_same_component_is_equal<T>(rect1: &Rectangle<T>, rect2: &Rectangle<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug + Num + NumAssignOps,
    {
        assert_eq!(rect1.width, rect2.width);
        assert_eq!(rect1.height, rect2.height);
        assert_eq!(rect1, rect2);
    }

    /// Rotate a rectangle twice is the same as the original
    fn assert_rotate_twice_is_same_as_original<T>(rect: &Rectangle<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug + Num + NumAssignOps,
    {
        let rotated_twice = rect.rotate_clockwise().rotate_clockwise();
        assert_rect_has_same_component_is_equal(rect, &rotated_twice);
        assert_eq!(rotated_twice, *rect);
    }
}
