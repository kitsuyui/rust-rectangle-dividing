use num_traits::{Float, Num, NumAssignOps, NumOps};

use crate::area::Area;
use crate::aspect_ratio::AspectRatio;
use crate::axis::{Axis, SizeForAxis};
use crate::component::Component;
use crate::dividing::VerticalDividingHelper;
use crate::point::{Edge, Point};
use crate::rectangle::{Rectangle, RectangleSize};
use crate::rotate::QuarterRotation;

/// axis aligned starting at x, y and ending at x + width, y + height (left to right, top to bottom)
#[derive(Debug, PartialEq, Clone)]
pub struct AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    pub point: Point<T>,
    pub rectangle: Rectangle<T>,
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd + Float,
{
    pub fn round(&self) -> Self {
        let p1 = self.edge_left_top().round(Edge::RightBottom);
        let p2 = self.edge_right_bottom().round(Edge::LeftTop);
        let width = p2.x() - p1.x();
        let height = p2.y() - p1.y();
        let rect = Rectangle::new(width, height);
        Self::new(&p1, &rect)
    }
}

impl<T> SizeForAxis<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps,
{
    fn size_for_axis(&self, axis: Axis) -> T {
        self.rectangle.size_for_axis(axis)
    }
}

/// A rectangle in 2D space with a width and height
impl<T> RectangleSize<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
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
    T: Copy + Num + NumAssignOps + NumOps,
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
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// A rectangle in 2D space constructor
    pub fn new(point: &Point<T>, rectangle: &Rectangle<T>) -> Self {
        Self {
            point: *point,
            rectangle: *rectangle,
        }
    }

    pub fn rect(&self) -> Rectangle<T> {
        self.rectangle
    }

    pub fn origin(&self) -> Point<T> {
        self.point
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + Float,
{
    pub fn from_two_point(p1: &Point<T>, p2: &Point<T>) -> Self {
        let vec = *p1 - *p2;
        let width = vec.x().abs();
        let height = vec.y().abs();
        let rect = Rectangle::new(width, height);

        Self::new(p1, &rect)
    }
}

impl<T> AspectRatio<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    fn aspect_ratio(&self) -> T {
        self.rectangle.aspect_ratio()
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd,
{
    pub(crate) fn edge_left_top(&self) -> Point<T> {
        self.point
    }
    pub(crate) fn edge_right_top(&self) -> Point<T> {
        Point::new(self.point.x() + self.rectangle.width(), self.point.y())
    }
    pub(crate) fn edge_left_bottom(&self) -> Point<T> {
        Point::new(self.point.x(), self.point.y() + self.rectangle.height())
    }
    pub(crate) fn edge_right_bottom(&self) -> Point<T> {
        Point::new(
            self.point.x() + self.rectangle.width(),
            self.point.y() + self.rectangle.height(),
        )
    }

    #[allow(dead_code)]
    pub(crate) fn edges(&self) -> Vec<Point<T>> {
        vec![
            self.edge_left_top(),
            self.edge_right_top(),
            self.edge_right_bottom(),
            self.edge_left_bottom(),
        ]
    }

    #[allow(dead_code)]
    pub(crate) fn includes(&self, p: &Point<T>) -> bool {
        p.x() > self.point.x()
            && p.x() < self.point.x() + self.rectangle.width()
            && p.y() > self.point.y()
            && p.y() < self.point.y() + self.rectangle.height()
    }

    #[allow(dead_code)]
    pub(crate) fn includes_or_on_the_boundary(&self, p: &Point<T>) -> bool {
        p.x() >= self.point.x()
            && p.x() <= self.point.x() + self.rectangle.width()
            && p.y() >= self.point.y()
            && p.y() <= self.point.y() + self.rectangle.height()
    }

    #[allow(dead_code)]
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        // if any of the edges of the other rectangle are inside this rectangle, then they overlap
        other.edges().iter().any(|p| self.includes(p))
    }

    #[allow(dead_code)]
    pub(crate) fn enclodes(&self, other: &Self) -> bool {
        // if all of the edges of the other rectangle are inside this rectangle, then they are enclosed
        other
            .edges()
            .iter()
            .all(|p| self.includes_or_on_the_boundary(p))
    }
}

/// area of an axis aligned rectangle
impl<T> Area<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps,
{
    fn area(&self) -> T {
        self.rectangle.area()
    }
}

/// Rotate an axis aligned rectangle by 90 degrees
impl<T> QuarterRotation for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps,
{
    fn rotate_clockwise(&self) -> Self {
        Self::new(
            &Point::new(self.y(), self.x()),
            &Rectangle::new(self.height(), self.width()),
        )
    }
}

impl<T> VerticalDividingHelper<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical_helper(&self, x: T) -> (AxisAlignedRectangle<T>, AxisAlignedRectangle<T>) {
        (
            Self::new(
                &Point::new(self.x(), self.y()),
                &Rectangle::new(x, self.height()),
            ),
            Self::new(
                &Point::new(self.x() + x, self.y()),
                &Rectangle::new(self.width() - x, self.height()),
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
        let result = AxisAlignedRectangle::new(&point, &rect);
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
        let result = AxisAlignedRectangle::new(&point, &rect).rotate_clockwise();
        assert_eq!(result.origin(), Point::new(3, 2));
        assert_eq!(result.rect(), Rectangle::new(5, 4));
    }

    #[test]
    fn test_area() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(&point, &rect).area();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_edges() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::new(&point, &rect).edges();
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
        let a_rect = AxisAlignedRectangle::new(&point, &rect);
        assert!(!a_rect.includes(&Point::new(1, 3)));
        assert!(!a_rect.includes(&Point::new(7, 3)));
        assert!(!a_rect.includes(&Point::new(6, 2)));
        assert!(!a_rect.includes(&Point::new(6, 9)));
    }

    #[test]
    fn test_overlaps() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let a_rect = AxisAlignedRectangle::new(&point, &rect);
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            &Point::new(1, 2),
            &Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            &Point::new(1, 4),
            &Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            &Point::new(1, 5),
            &Rectangle::new(4, 5)
        )));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::new(
            &Point::new(1, 6),
            &Rectangle::new(4, 5)
        )));
    }
}
