use crate::component::Component;
use crate::dividing::Dividing;
use crate::point::Point;
use crate::rectangle::{Area, Rectangle, RectangleSize};
use crate::rotate::Rotate;

/// axis aligned starting at x, y and ending at x + width, y + height (left to right, top to bottom)
pub struct AxisAlignedRectangle<T>
where
    T: Copy,
{
    pub point: Point<T>,
    pub rectangle: Rectangle<T>,
}

impl<T> Clone for AxisAlignedRectangle<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
            rectangle: self.rectangle.clone(),
        }
    }
}

/// A rectangle in 2D space with a width and height
impl<T> RectangleSize<T> for AxisAlignedRectangle<T>
where
    T: Copy,
{
    fn width(&self) -> T {
        self.rectangle.width()
    }
    fn height(&self) -> T {
        self.rectangle.height()
    }
}

impl<T> Component<T> for AxisAlignedRectangle<T>
where
    T: Copy,
{
    fn x(&self) -> T {
        self.point.x()
    }

    fn y(&self) -> T {
        self.point.y()
    }
}

/// A rectangle in 2D space constructor
impl<T> AxisAlignedRectangle<T>
where
    T: Copy,
{
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

impl<T> Dividing<T> for AxisAlignedRectangle<T>
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
        (
            Self::new(
                Point::new(self.x(), self.y()),
                Rectangle::new(x, self.height()),
            ),
            Self::new(
                Point::new(self.x() + x, self.y()),
                Rectangle::new(self.width() - x, self.height()),
            ),
        )
    }

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
        (
            Self::new(
                Point::new(self.x(), self.y()),
                Rectangle::new(self.width(), y),
            ),
            Self::new(
                Point::new(self.x(), self.y() + y),
                Rectangle::new(self.width(), self.height() - y),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::dividing::DividingDirection;

    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect);
        assert_eq!(result.x(), 2);
        assert_eq!(result.y(), 3);
        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 5);
    }

    #[test]
    fn test_rotate() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect).rotate();
        assert_eq!(result.x(), 3);
        assert_eq!(result.y(), 2);
        assert_eq!(result.width(), 5);
        assert_eq!(result.height(), 4);
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
        assert_eq!(rect_a.x(), 2);
        assert_eq!(rect_a.y(), 3);
        assert_eq!(rect_a.width(), 2);
        assert_eq!(rect_a.height(), 5);
        assert_eq!(rect_b.x(), 4);
        assert_eq!(rect_b.y(), 3);
        assert_eq!(rect_b.width(), 2);
        assert_eq!(rect_b.height(), 5);
    }

    #[test]
    fn test_divide_horizontal() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_horizontal(2);
        assert_eq!(rect_a.x(), 2);
        assert_eq!(rect_a.y(), 3);
        assert_eq!(rect_a.width(), 4);
        assert_eq!(rect_a.height(), 2);
        assert_eq!(rect_b.x(), 2);
        assert_eq!(rect_b.y(), 5);
        assert_eq!(rect_b.width(), 4);
        assert_eq!(rect_b.height(), 3);
    }

    #[test]
    fn test_divide_nth() {
        // test vertical
        let point = Point::new(2, 3);
        let rect = Rectangle::new(6, 2);
        let a_rect = AxisAlignedRectangle::new(point, rect);
        let divided = a_rect.divide_by_values(vec![1, 2], &DividingDirection::Vertical);
        assert_eq!(divided[0].x(), 2);
        assert_eq!(divided[0].y(), 3);
        assert_eq!(divided[0].width(), 1);
        assert_eq!(divided[0].height(), 2);
        assert_eq!(divided[1].x(), 3);
        assert_eq!(divided[1].y(), 3);
        assert_eq!(divided[1].width(), 2);
        assert_eq!(divided[1].height(), 2);
        assert_eq!(divided[2].x(), 5);
        assert_eq!(divided[2].y(), 3);
        assert_eq!(divided[2].width(), 3);
        assert_eq!(divided[2].height(), 2);
        assert_eq!(divided.len(), 3);
        // sum of divided rectangles should equal original rectangle
        assert_eq!(
            divided[0].width() + divided[1].width() + divided[2].width(),
            a_rect.width()
        );
        // all divided rectangles should have the same height
        assert_eq!(divided[0].height(), a_rect.height());
        assert_eq!(divided[1].height(), a_rect.height());
        assert_eq!(divided[2].height(), a_rect.height());
        // the sum of the x and width of the  divided rectangle should equal the x of the next divided rectangle
        assert_eq!(divided[0].x() + divided[0].width(), divided[1].x());
        assert_eq!(divided[1].x() + divided[1].width(), divided[2].x());
        assert_eq!(
            a_rect.x() + a_rect.width(),
            divided[2].x() + divided[2].width()
        );

        // test horizontal
        let point = Point::new(2, 3);
        let rect = Rectangle::new(2, 6);
        let a_rect = AxisAlignedRectangle::new(point, rect);
        let divided = a_rect.divide_by_values(vec![3, 2], &DividingDirection::Horizontal);
        assert_eq!(divided[0].x(), 2);
        assert_eq!(divided[0].y(), 3);
        assert_eq!(divided[0].width(), 2);
        assert_eq!(divided[0].height(), 3);
        assert_eq!(divided[1].x(), 2);
        assert_eq!(divided[1].y(), 6);
        assert_eq!(divided[1].width(), 2);
        assert_eq!(divided[1].height(), 2);
        assert_eq!(divided[2].x(), 2);
        assert_eq!(divided[2].y(), 8);
        assert_eq!(divided[2].width(), 2);
        assert_eq!(divided[2].height(), 1);
        assert_eq!(divided.len(), 3);
        // sum of divided rectangles should equal original rectangle
        assert_eq!(
            divided[0].height() + divided[1].height() + divided[2].height(),
            a_rect.height()
        );
        // all divided rectangles should have the same width
        assert_eq!(divided[0].width(), a_rect.width());
        assert_eq!(divided[1].width(), a_rect.width());
        assert_eq!(divided[2].width(), a_rect.width());
        // the sum of the y and height of the  divided rectangle should equal the y of the next divided rectangle
        assert_eq!(divided[0].y() + divided[0].height(), divided[1].y());
        assert_eq!(divided[1].y() + divided[1].height(), divided[2].y());
        assert_eq!(
            a_rect.y() + a_rect.height(),
            divided[2].y() + divided[2].height()
        );
    }
}
