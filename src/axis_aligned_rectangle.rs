use crate::area::Area;
use crate::aspect_ratio::AspectRatio;
use crate::axis::{Axis, SizeForAxis};
use crate::component::Component;
use crate::dividing::VerticalDividingHelper;
use crate::point::Point;
use crate::rectangle::{Rectangle, RectangleSize};
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

impl<T> AspectRatio<T> for AxisAlignedRectangle<T>
where
    T: Copy + std::ops::Div<Output = T>,
{
    fn aspect_ratio(&self) -> T {
        self.rectangle.aspect_ratio()
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
