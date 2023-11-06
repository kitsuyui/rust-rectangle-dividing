use crate::axis::{Axis, SizeForAxis};
use crate::dividing::Dividing;
use crate::rotate::Rotate;
/// rectangle in 2D space with a width and height

#[derive(Debug, PartialEq, Clone)]
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
    fn size_for_axis(&self, axis: &Axis) -> T {
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
impl<T> Rotate for Rectangle<T>
where
    T: Copy,
{
    fn rotate(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}

impl<T> Dividing<T> for Rectangle<T>
where
    T: Copy + std::ops::Sub<Output = T>,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(x, self.height),
            Self::new(self.width - x, self.height),
        )
    }

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(self.width, y),
            Self::new(self.width, self.height - y),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;

    use super::*;

    #[test]
    fn test_new() {
        let result = Rectangle::new(2, 2);
        assert_eq!(result.width, 2);
        assert_eq!(result.height, 2);
    }

    fn assert_rotate_twice_is_original<T>(rect: Rectangle<T>)
    where
        T: Copy + PartialEq + std::fmt::Debug,
    {
        let result = rect.rotate().rotate();
        assert_eq!(result.width, rect.width);
        assert_eq!(result.height, rect.height);
    }

    #[test]
    fn test_rotate() {
        let rect = Rectangle::new(2, 3);
        let result = rect.rotate();
        assert_eq!(result.width, 3);
        assert_eq!(result.height, 2);
        assert_rotate_twice_is_original(rect);
    }

    #[test]
    fn test_area() {
        let result = Rectangle::new(2, 3).area();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_divide_vertical() {
        let (rect_a, rect_b) = Rectangle::new(4, 2).divide_vertical(1);
        assert_eq!(rect_a.width, 1);
        assert_eq!(rect_a.height, 2);
        assert_eq!(rect_b.width, 3);
        assert_eq!(rect_b.height, 2);
    }

    #[test]
    fn test_divide_horizontal() {
        let (rect_a, rect_b) = Rectangle::new(2, 4).divide_horizontal(1);
        assert_eq!(rect_a.width, 2);
        assert_eq!(rect_a.height, 1);
        assert_eq!(rect_b.width, 2);
        assert_eq!(rect_b.height, 3);
    }

    #[test]
    fn test_divide() {
        let (rect_a, rect_b) = Rectangle::new(4, 2).divide(1, &Axis::Vertical);
        assert_eq!(rect_a.width, 1);
        assert_eq!(rect_a.height, 2);
        assert_eq!(rect_b.width, 3);
        assert_eq!(rect_b.height, 2);

        let (rect_a, rect_b) = Rectangle::new(2, 4).divide(1, &Axis::Horizontal);
        assert_eq!(rect_a.width, 2);
        assert_eq!(rect_a.height, 1);
        assert_eq!(rect_b.width, 2);
        assert_eq!(rect_b.height, 3);
    }

    #[test]
    fn test_divide_nth() {
        let rect = Rectangle::new(6, 2);
        let divided = rect.divide_by_values(vec![1, 2], &Axis::Vertical);
        assert_eq!(divided[0].width, 1);
        assert_eq!(divided[0].height, 2);
        assert_eq!(divided[1].width, 2);
        assert_eq!(divided[1].height, 2);
        assert_eq!(divided[2].width, 3);
        assert_eq!(divided[2].height, 2);

        let rect = Rectangle::new(2, 6);
        let divided = rect.divide_by_values(vec![3, 2], &Axis::Horizontal);
        assert_eq!(divided[0].width, 2);
        assert_eq!(divided[0].height, 3);
        assert_eq!(divided[1].width, 2);
        assert_eq!(divided[1].height, 2);
        assert_eq!(divided[2].width, 2);
        assert_eq!(divided[2].height, 1);
    }

    #[test]
    fn test_divide_by_weights() {
        let rect = Rectangle::new(6, 2);
        // values
        let divided1 = rect.divide_by_values(vec![1, 2], &Axis::Vertical);

        let rect = Rectangle::new(6, 2);
        let divided2 = rect.divide_by_weights(vec![2, 4, 6], &Axis::Vertical);
        assert_eq!(divided1, divided2);
    }
}
