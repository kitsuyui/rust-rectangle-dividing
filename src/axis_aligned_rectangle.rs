use crate::dividing::Dividing;
use crate::point::Point;
use crate::rectangle::{Area, Rectangle};
use crate::rotate::Rotate;

/// axis aligned starting at x, y and ending at x + width, y + height (left to right, top to bottom)
pub struct AxisAlignedRectangle<T> {
    pub point: Point<T>,
    pub rectangle: Rectangle<T>,
}

/// A rectangle in 2D space constructor
impl<T> AxisAlignedRectangle<T> {
    pub fn new(point: Point<T>, rectangle: Rectangle<T>) -> Self {
        Self { point, rectangle }
    }
}

/// area of an axis aligned rectangle
impl<T: std::ops::Mul<Output = T> + Copy> Area<T> for AxisAlignedRectangle<T> {
    fn area(&self) -> T {
        self.rectangle.area()
    }
}

/// Rotate an axis aligned rectangle by 90 degrees
impl<T: Copy> Rotate for AxisAlignedRectangle<T> {
    fn rotate(&self) -> Self {
        Self {
            point: self.point.rotate(),
            rectangle: self.rectangle.rotate(),
        }
    }
}

impl<T: std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy> Dividing<T>
    for AxisAlignedRectangle<T>
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
        (
            Self::new(
                Point::new(self.point.x, self.point.y),
                Rectangle::new(x, self.rectangle.height),
            ),
            Self::new(
                Point::new(self.point.x + x, self.point.y),
                Rectangle::new(self.rectangle.width - x, self.rectangle.height),
            ),
        )
    }

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
        (
            Self::new(
                Point::new(self.point.x, self.point.y),
                Rectangle::new(self.rectangle.width, y),
            ),
            Self::new(
                Point::new(self.point.x, self.point.y + y),
                Rectangle::new(self.rectangle.width, self.rectangle.height - y),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect);
        assert_eq!(result.point.x, 2);
        assert_eq!(result.point.y, 3);
        assert_eq!(result.rectangle.width, 4);
        assert_eq!(result.rectangle.height, 5);
    }

    #[test]
    fn test_rotate() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect).rotate();
        assert_eq!(result.point.x, 3);
        assert_eq!(result.point.y, 2);
        assert_eq!(result.rectangle.width, 5);
        assert_eq!(result.rectangle.height, 4);
    }

    #[test]
    fn test_area() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect).area();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_divide_vertical() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_vertical(2);
        assert_eq!(rect_a.point.x, 2);
        assert_eq!(rect_a.point.y, 3);
        assert_eq!(rect_a.rectangle.width, 2);
        assert_eq!(rect_a.rectangle.height, 5);
        assert_eq!(rect_b.point.x, 4);
        assert_eq!(rect_b.point.y, 3);
        assert_eq!(rect_b.rectangle.width, 2);
        assert_eq!(rect_b.rectangle.height, 5);
    }

    #[test]
    fn test_divide_horizontal() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_horizontal(2);
        assert_eq!(rect_a.point.x, 2);
        assert_eq!(rect_a.point.y, 3);
        assert_eq!(rect_a.rectangle.width, 4);
        assert_eq!(rect_a.rectangle.height, 2);
        assert_eq!(rect_b.point.x, 2);
        assert_eq!(rect_b.point.y, 5);
        assert_eq!(rect_b.rectangle.width, 4);
        assert_eq!(rect_b.rectangle.height, 3);
    }
}
