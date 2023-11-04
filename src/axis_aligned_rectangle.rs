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
        AxisAlignedRectangle { point, rectangle }
    }
}

/// area of an axis aligned rectangle
impl<T: std::ops::Mul<Output = T> + Copy> Area<T> for AxisAlignedRectangle<T> {
    fn area(&self) -> T {
        self.rectangle.area()
    }
}

/// Rotate an axis aligned rectangle by 90 degrees
impl Rotate for AxisAlignedRectangle<i32> {
    fn rotate(&self) -> Self {
        AxisAlignedRectangle {
            point: self.point.rotate(),
            rectangle: self.rectangle.rotate(),
        }
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
}
