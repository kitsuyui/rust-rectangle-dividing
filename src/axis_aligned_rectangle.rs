use num_traits::{Float, Num, NumAssignOps, NumOps};

use crate::area::Area;
use crate::aspect_ratio::AspectRatio;
use crate::axis::{Axis, SizeForAxis};
use crate::component::Component;
use crate::point::{Edge, Point};
use crate::rectangle::VerticalDividingHelper;
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
    /// Round the coordinates of the rectangle to the nearest integers
    /// This is useful for snapping the rectangle to pixel boundaries.
    ///
    /// Sub-pixel rectangles (width or height < 1.0) clamp to zero size after
    /// rounding because the ceil-of-left-top can exceed the floor-of-right-bottom.
    pub fn round(&self) -> Self {
        let p1 = self.edge_left_top().round(Edge::RightBottom);
        let p2 = self.edge_right_bottom().round(Edge::LeftTop);
        let width = (p2.x() - p1.x()).max(T::zero());
        let height = (p2.y() - p1.y()).max(T::zero());
        let rect = Rectangle::new(width, height);
        Self::new(&p1, &rect)
    }
}

impl<T> SizeForAxis<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps,
{
    /// Get the size of the rectangle along the specified axis
    fn size_for_axis(&self, axis: Axis) -> T {
        self.rectangle.size_for_axis(axis)
    }
}

/// rectangle size implementation for axis aligned rectangle
impl<T> RectangleSize<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// Get the width of the rectangle
    fn width(&self) -> T {
        self.rectangle.width()
    }
    /// Get the height of the rectangle
    fn height(&self) -> T {
        self.rectangle.height()
    }
}

impl<T> Component<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// Get the x coordinate of the rectangle
    fn x(&self) -> T {
        self.point.x()
    }

    /// Get the y coordinate of the rectangle
    fn y(&self) -> T {
        self.point.y()
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// Get the x coordinate of the rectangle
    fn min_x(&self) -> T {
        self.point.x()
    }
    /// Get the x coordinate of the rectangle plus its width
    fn max_x(&self) -> T {
        self.point.x() + self.rectangle.width()
    }

    /// Get the y coordinate of the rectangle
    fn min_y(&self) -> T {
        self.point.y()
    }

    /// Get the y coordinate of the rectangle plus its height
    fn max_y(&self) -> T {
        self.point.y() + self.rectangle.height()
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd,
{
    /// Create a new axis aligned rectangle
    pub fn new(point: &Point<T>, rectangle: &Rectangle<T>) -> Self {
        Self {
            point: *point,
            rectangle: *rectangle,
        }
    }
    /// Create a new axis aligned rectangle from 4 values
    pub(crate) fn from4values(x: T, y: T, width: T, height: T) -> Self {
        Self::new(&Point::new(x, y), &Rectangle::new(width, height))
    }

    /// Get the rectangle
    pub fn rect(&self) -> Rectangle<T> {
        self.rectangle
    }

    /// Get the origin point of the rectangle
    pub fn origin(&self) -> Point<T> {
        self.point
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + Float,
{
    /// Create an axis aligned rectangle from two points.
    ///
    /// The result covers the region `(min(p1, p2)) -> (max(p1, p2))`,
    /// independent of which point is given as `p1` or `p2`.
    pub fn from_two_point(p1: &Point<T>, p2: &Point<T>) -> Self {
        let min_x = p1.x().min(p2.x());
        let min_y = p1.y().min(p2.y());
        let max_x = p1.x().max(p2.x());
        let max_y = p1.y().max(p2.y());
        let width = max_x - min_x;
        let height = max_y - min_y;
        let rect = Rectangle::new(width, height);

        Self::new(&Point::new(min_x, min_y), &rect)
    }
}

impl<T> AspectRatio<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps,
{
    /// Get the aspect ratio of the rectangle
    fn aspect_ratio(&self) -> T {
        self.rectangle.aspect_ratio()
    }
}

impl<T> AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd,
{
    pub(crate) fn edge_left_top(&self) -> Point<T> {
        Point::new(self.min_x(), self.min_y())
    }
    pub(crate) fn edge_right_top(&self) -> Point<T> {
        Point::new(self.max_x(), self.min_y())
    }
    pub(crate) fn edge_left_bottom(&self) -> Point<T> {
        Point::new(self.min_x(), self.max_y())
    }
    pub(crate) fn edge_right_bottom(&self) -> Point<T> {
        Point::new(self.max_x(), self.max_y())
    }

    pub(crate) fn edges(&self) -> Vec<Point<T>> {
        vec![
            self.edge_left_top(),
            self.edge_right_top(),
            self.edge_right_bottom(),
            self.edge_left_bottom(),
        ]
    }

    /// Check if the point is strictly inside the rectangle (excluding the boundary).
    /// Use [`Self::includes_or_on_the_boundary`] when the boundary should count as inside.
    #[allow(dead_code)]
    pub(crate) fn includes(&self, p: &Point<T>) -> bool {
        // x
        if p.x() <= self.min_x() || p.x() >= self.max_x() {
            return false;
        }
        // y
        if p.y() <= self.min_y() || p.y() >= self.max_y() {
            return false;
        }
        true
    }

    pub(crate) fn includes_or_on_the_boundary(&self, p: &Point<T>) -> bool {
        // x
        if p.x() < self.min_x() || p.x() > self.max_x() {
            return false;
        }
        // y
        if p.y() < self.min_y() || p.y() > self.max_y() {
            return false;
        }
        true
    }

    #[allow(dead_code)]
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.min_x() < other.max_x()
            && self.max_x() > other.min_x()
            && self.min_y() < other.max_y()
            && self.max_y() > other.min_y()
    }

    #[allow(dead_code)]
    pub(crate) fn encloses(&self, other: &Self) -> bool {
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
    T: Copy + Num + NumAssignOps + PartialOrd,
{
    fn rotate_clockwise(&self) -> Self {
        Self::from4values(self.y(), self.x(), self.height(), self.width())
    }
}

impl<T> VerticalDividingHelper<T> for AxisAlignedRectangle<T>
where
    T: Copy + Num + NumAssignOps + NumOps + PartialOrd,
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
        // let point = Point::new(2, 3);
        // let rect = Rectangle::new(4, 5);
        let result = AxisAlignedRectangle::from4values(2, 3, 4, 5).rotate_clockwise();
        assert_eq!(result.origin(), Point::new(3, 2));
        assert_eq!(result.rect(), Rectangle::new(5, 4));
    }

    #[test]
    fn test_area() {
        let result = AxisAlignedRectangle::from4values(2, 3, 4, 5).area();
        // 4 * 5 = 20
        assert_eq!(result, 20);
    }

    #[test]
    fn test_edges() {
        let result = AxisAlignedRectangle::from4values(2, 3, 4, 5).edges();
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], Point::new(2, 3));
        assert_eq!(result[1], Point::new(6, 3));
        assert_eq!(result[2], Point::new(6, 8));
        assert_eq!(result[3], Point::new(2, 8));
    }

    #[test]
    fn test_include() {
        let a_rect = AxisAlignedRectangle::from4values(2, 3, 4, 5);
        assert!(a_rect.includes(&Point::new(3, 4)));
        assert!(a_rect.includes(&Point::new(5, 7)));
        assert!(!a_rect.includes(&Point::new(1, 3)));
        assert!(!a_rect.includes(&Point::new(7, 3)));
        assert!(!a_rect.includes(&Point::new(6, 2)));
        assert!(!a_rect.includes(&Point::new(6, 9)));

        // Boundary points are strictly excluded (corners and edge midpoints).
        assert!(!a_rect.includes(&Point::new(2, 3)));
        assert!(!a_rect.includes(&Point::new(6, 3)));
        assert!(!a_rect.includes(&Point::new(2, 8)));
        assert!(!a_rect.includes(&Point::new(6, 8)));
        assert!(!a_rect.includes(&Point::new(4, 3)));
        assert!(!a_rect.includes(&Point::new(6, 5)));
        assert!(!a_rect.includes(&Point::new(4, 8)));
        assert!(!a_rect.includes(&Point::new(2, 5)));
    }

    #[test]
    fn test_includes_or_on_the_boundary() {
        let a_rect = AxisAlignedRectangle::from4values(2, 3, 4, 5);
        // Strict interior points are included.
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(3, 4)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(5, 7)));
        // Boundary points (corners and edge midpoints) are also included.
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(2, 3)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(6, 3)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(2, 8)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(6, 8)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(4, 3)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(6, 5)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(4, 8)));
        assert!(a_rect.includes_or_on_the_boundary(&Point::new(2, 5)));
        // Outside points are excluded.
        assert!(!a_rect.includes_or_on_the_boundary(&Point::new(1, 3)));
        assert!(!a_rect.includes_or_on_the_boundary(&Point::new(7, 3)));
        assert!(!a_rect.includes_or_on_the_boundary(&Point::new(6, 2)));
        assert!(!a_rect.includes_or_on_the_boundary(&Point::new(6, 9)));
    }

    #[test]
    fn test_overlaps() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let a_rect = AxisAlignedRectangle::new(&point, &rect);
        assert!(a_rect.overlaps(&AxisAlignedRectangle::from4values(1, 2, 4, 5)));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::from4values(1, 4, 4, 5)));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::from4values(1, 5, 4, 5)));
        assert!(a_rect.overlaps(&AxisAlignedRectangle::from4values(1, 6, 4, 5)));

        assert!(!a_rect.overlaps(&AxisAlignedRectangle::from4values(0, 0, 1, 1)));
        assert!(!a_rect.overlaps(&AxisAlignedRectangle::from4values(5, 8, 6, 9)));
    }

    #[test]
    fn test_overlaps_detects_crossing_rectangles_without_corner_inclusion() {
        let horizontal = AxisAlignedRectangle::from4values(0, 4, 10, 2);
        let vertical = AxisAlignedRectangle::from4values(4, 0, 2, 10);
        assert!(horizontal.overlaps(&vertical));
        assert!(vertical.overlaps(&horizontal));
    }

    #[test]
    fn test_overlaps_detects_enclosing_rectangles() {
        let outer = AxisAlignedRectangle::from4values(0, 0, 10, 10);
        let inner = AxisAlignedRectangle::from4values(2, 3, 4, 5);
        assert!(outer.overlaps(&inner));
        assert!(inner.overlaps(&outer));
    }

    #[test]
    fn test_overlaps_excludes_boundary_touching_rectangles() {
        let a_rect = AxisAlignedRectangle::from4values(2, 3, 4, 5);
        assert!(!a_rect.overlaps(&AxisAlignedRectangle::from4values(6, 3, 4, 5)));
        assert!(!a_rect.overlaps(&AxisAlignedRectangle::from4values(2, 8, 4, 5)));
    }

    #[test]
    fn test_from_two_point_is_order_independent() {
        // The four orderings of two points must all produce the same rectangle:
        // origin (1.0, 2.0) with width 4.0 and height 3.0.
        let a = Point::new(1.0_f64, 2.0_f64);
        let b = Point::new(5.0_f64, 5.0_f64);
        let c = Point::new(1.0_f64, 5.0_f64);
        let d = Point::new(5.0_f64, 2.0_f64);

        let expected_origin = Point::new(1.0_f64, 2.0_f64);
        let expected_rect = Rectangle::new(4.0_f64, 3.0_f64);

        for (p1, p2) in [(a, b), (b, a), (c, d), (d, c)] {
            let r = AxisAlignedRectangle::from_two_point(&p1, &p2);
            assert_eq!(r.origin(), expected_origin);
            assert_eq!(r.rect(), expected_rect);
        }
    }

    #[test]
    fn test_from_two_point_with_p1_greater_than_p2() {
        // Regression: previously the origin was unconditionally `p1`, so
        // `from_two_point((5, 5), (0, 0))` produced a rectangle covering
        // (5, 5)-(10, 10) instead of the expected (0, 0)-(5, 5).
        let p1 = Point::new(5.0_f64, 5.0_f64);
        let p2 = Point::new(0.0_f64, 0.0_f64);
        let r = AxisAlignedRectangle::from_two_point(&p1, &p2);
        assert_eq!(r.origin(), Point::new(0.0_f64, 0.0_f64));
        assert_eq!(r.rect(), Rectangle::new(5.0_f64, 5.0_f64));
    }

    #[test]
    fn test_from_two_point_with_degenerate_points() {
        // Same point twice produces a zero-sized rectangle anchored there.
        let p = Point::new(3.0_f64, 4.0_f64);
        let r = AxisAlignedRectangle::from_two_point(&p, &p);
        assert_eq!(r.origin(), p);
        assert_eq!(r.rect(), Rectangle::new(0.0_f64, 0.0_f64));
    }

    #[test]
    #[should_panic(expected = "width must be non-negative")]
    fn test_from4values_rejects_negative_width() {
        AxisAlignedRectangle::from4values(0, 0, -10, 5);
    }

    #[test]
    #[should_panic(expected = "height must be non-negative")]
    fn test_from4values_rejects_negative_height() {
        AxisAlignedRectangle::from4values(0, 0, 10, -5);
    }

    #[test]
    fn test_round_normal_rect() {
        // Integer-aligned rect: round is a no-op.
        let r = AxisAlignedRectangle::from4values(1.0_f64, 2.0, 3.0, 4.0);
        let rounded = r.round();
        assert_eq!(rounded.x(), 1.0);
        assert_eq!(rounded.y(), 2.0);
        assert_eq!(rounded.width(), 3.0);
        assert_eq!(rounded.height(), 4.0);
    }

    #[test]
    fn test_round_fractional_rect() {
        // Rect with fractional parts snaps outward via ceil(left-top) / floor(right-bottom).
        // x=0.3, y=0.4, width=2.5, height=3.2  →  right=2.8, bottom=3.6
        // p1 = (ceil(0.3), ceil(0.4)) = (1.0, 1.0)
        // p2 = (floor(2.8), floor(3.6)) = (2.0, 3.0)
        // width = 1.0, height = 2.0
        let r = AxisAlignedRectangle::from4values(0.3_f64, 0.4, 2.5, 3.2);
        let rounded = r.round();
        assert_eq!(rounded.x(), 1.0);
        assert_eq!(rounded.y(), 1.0);
        assert_eq!(rounded.width(), 1.0);
        assert_eq!(rounded.height(), 2.0);
    }

    #[test]
    fn test_round_sub_pixel_width_clamps_to_zero() {
        // Sub-pixel width: x=1.6, width=0.1  →  right=1.7
        // p1.x = ceil(1.6) = 2.0, p2.x = floor(1.7) = 1.0
        // raw diff = -1.0; must clamp to 0.0 (not go negative).
        let r = AxisAlignedRectangle::from4values(1.6_f64, 0.0, 0.1, 2.0);
        let rounded = r.round();
        assert_eq!(rounded.width(), 0.0, "sub-pixel width must clamp to 0");
        assert!(
            rounded.height() >= 0.0,
            "height must be non-negative after round"
        );
    }

    #[test]
    fn test_round_sub_pixel_height_clamps_to_zero() {
        // Sub-pixel height: y=1.6, height=0.1  →  bottom=1.7
        // p1.y = ceil(1.6) = 2.0, p2.y = floor(1.7) = 1.0
        // raw diff = -1.0; must clamp to 0.0.
        let r = AxisAlignedRectangle::from4values(0.0, 1.6_f64, 2.0, 0.1);
        let rounded = r.round();
        assert_eq!(rounded.height(), 0.0, "sub-pixel height must clamp to 0");
        assert!(
            rounded.width() >= 0.0,
            "width must be non-negative after round"
        );
    }
}
