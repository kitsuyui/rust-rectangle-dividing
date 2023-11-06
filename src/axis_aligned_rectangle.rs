use crate::axis::{Axis, SizeForAxis};
use crate::component::Component;
use crate::dividing::VerticalDividingHelper;
use crate::point::Point;
use crate::rectangle::{Area, Rectangle, RectangleSize};
use crate::rotate::QuarterRotation;

/// axis aligned starting at x, y and ending at x + width, y + height (left to right, top to bottom)
#[derive(Debug, PartialEq, Clone)]
pub struct AxisAlignedRectangle<T>
where
    T: Copy,
{
    pub point: Point<T>,
    pub rectangle: Rectangle<T>,
}

impl<T> SizeForAxis<T> for AxisAlignedRectangle<T>
where
    T: Copy,
{
    fn size_for_axis(&self, axis: Axis) -> T {
        self.rectangle.size_for_axis(axis)
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

impl<T> AxisAlignedRectangle<T>
where
    T: Copy,
{
    /// A rectangle in 2D space constructor
    pub fn new(point: Point<T>, rectangle: Rectangle<T>) -> Self {
        Self { point, rectangle }
    }

    pub fn rect(&self) -> Rectangle<T> {
        self.rectangle
    }

    pub fn origin(&self) -> Point<T> {
        self.point
    }
}

#[cfg(test)]
impl<T> AxisAlignedRectangle<T>
where
    T: Copy
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::cmp::PartialOrd,
{
    pub(crate) fn edges(&self) -> Vec<Point<T>> {
        vec![
            self.point,
            Point::new(self.point.x() + self.rectangle.width(), self.point.y()),
            Point::new(
                self.point.x() + self.rectangle.width(),
                self.point.y() + self.rectangle.height(),
            ),
            Point::new(self.point.x(), self.point.y() + self.rectangle.height()),
        ]
    }

    pub(crate) fn includes(&self, p: &Point<T>) -> bool {
        p.x() > self.point.x()
            && p.x() < self.point.x() + self.rectangle.width()
            && p.y() > self.point.y()
            && p.y() < self.point.y() + self.rectangle.height()
    }

    pub(crate) fn includes_or_on_the_boundary(&self, p: &Point<T>) -> bool {
        p.x() >= self.point.x()
            && p.x() <= self.point.x() + self.rectangle.width()
            && p.y() >= self.point.y()
            && p.y() <= self.point.y() + self.rectangle.height()
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        // if any of the edges of the other rectangle are inside this rectangle, then they overlap
        other.edges().iter().any(|p| self.includes(p))
    }

    pub(crate) fn enclodes(&self, other: &Self) -> bool {
        // if all of the edges of the other rectangle are inside this rectangle, then they are enclosed
        other
            .edges()
            .iter()
            .all(|p| self.includes_or_on_the_boundary(p))
    }
}

/// area of an axis aligned rectangle
impl<T: std::ops::Mul<Output = T> + Copy> Area<T> for AxisAlignedRectangle<T> {
    fn area(&self) -> T {
        self.rectangle.area()
    }
}

/// Rotate an axis aligned rectangle by 90 degrees
impl<T: Copy> QuarterRotation for AxisAlignedRectangle<T> {
    fn rotate_clockwise(&self) -> Self {
        Self::new(
            Point::new(self.y(), self.x()),
            Rectangle::new(self.height(), self.width()),
        )
    }
}

impl<T> VerticalDividingHelper<T> for AxisAlignedRectangle<T>
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical_helper(&self, x: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
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
}

#[cfg(test)]
mod tests {
    use crate::axis::Axis;
    use crate::dividing::Dividing;

    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect);
        assert_eq!(result.origin(), point);
        assert_eq!(result.rect(), rect);
        assert_eq!(result.x(), 2);
        assert_eq!(result.y(), 3);
        assert_eq!(result.width(), 4);
        assert_eq!(result.height(), 5);
    }

    #[test]
    fn test_rotate() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect).rotate_clockwise();
        assert_eq!(result.origin(), Point::new(3, 2));
        assert_eq!(result.rect(), Rectangle::new(5, 4));
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
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(2, 5));
        assert_eq!(rect_b.origin(), Point::new(4, 3));
        assert_eq!(rect_b.rect(), Rectangle::new(2, 5));

        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_vertical(1);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(1, 5));
        assert_eq!(rect_b.origin(), Point::new(3, 3));
        assert_eq!(rect_b.rect(), Rectangle::new(3, 5));
    }

    #[test]
    fn test_divide_horizontal() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_horizontal(1);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(4, 1));
        assert_eq!(rect_b.origin(), Point::new(2, 4));
        assert_eq!(rect_b.rect(), Rectangle::new(4, 4));

        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(point, rect).divide_horizontal(2);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(4, 2));
        assert_eq!(rect_b.origin(), Point::new(2, 5));
        assert_eq!(rect_b.rect(), Rectangle::new(4, 3));
    }

    #[test]
    fn test_divide_nth() {
        // test vertical
        let point = Point::new(2, 3);
        let rect = Rectangle::new(6, 2);
        let a_rect = AxisAlignedRectangle::new(point, rect);
        let divided = a_rect.divide_by_values_and_axis(&vec![1, 2], Axis::Vertical);
        assert_eq!(divided[0].origin(), point);
        assert_eq!(divided[0].rect(), Rectangle::new(1, 2));
        assert_eq!(divided[1].origin(), Point::new(3, 3));
        assert_eq!(divided[1].rect(), Rectangle::new(2, 2));
        assert_eq!(divided[2].origin(), Point::new(5, 3));
        assert_eq!(divided[2].rect(), Rectangle::new(3, 2));
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
        let divided = a_rect.divide_by_values_and_axis(&vec![3, 2], Axis::Horizontal);
        assert_eq!(divided[0].origin(), point);
        assert_eq!(divided[0].rect(), Rectangle::new(2, 3));
        assert_eq!(divided[1].origin(), Point::new(2, 6));
        assert_eq!(divided[1].rect(), Rectangle::new(2, 2));
        assert_eq!(divided[2].origin(), Point::new(2, 8));
        assert_eq!(divided[2].rect(), Rectangle::new(2, 1));
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

    #[test]
    fn test_edges() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(point, rect).edges();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], point);
        assert_eq!(result[1], Point::new(6, 3));
        assert_eq!(result[2], Point::new(6, 8));
        assert_eq!(result[3], Point::new(2, 8));
    }

    #[test]
    fn test_include() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let a_rect = AxisAlignedRectangle::new(point, rect);
        assert!(!a_rect.includes(&Point::new(1, 3)));
        assert!(!a_rect.includes(&Point::new(7, 3)));
        assert!(!a_rect.includes(&Point::new(6, 2)));
        assert!(!a_rect.includes(&Point::new(6, 9)));
    }

    #[test]
    fn test_overlaps() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let a_rect = AxisAlignedRectangle::new(point, rect);
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            Point::new(1, 2),
            Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            Point::new(1, 4),
            Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            Point::new(1, 5),
            Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            Point::new(1, 6),
            Rectangle::new(4, 5)
        )));
    }
}
