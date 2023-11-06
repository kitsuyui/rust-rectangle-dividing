use crate::{
    area::Area,
    axis::{Axis, SizeForAxis},
    rectangle::RectangleSize,
    rotate::QuarterRotation,
};

pub trait Dividing<T> {
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (Self, Self)
    where
        Self: Sized;

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (Self, Self)
    where
        Self: Sized;

    /// dividing a rectangle into two rectangles specified by axis
    fn divide(&self, v: T, axis: Axis) -> (Self, Self)
    where
        Self: Sized,
    {
        match axis {
            Axis::Vertical => self.divide_vertical(v),
            Axis::Horizontal => self.divide_horizontal(v),
        }
    }

    /// dividing a rectangle into specified number of rectangles specified by axis
    fn divide_by_values_and_axis(&self, values: &Vec<T>, axis: Axis) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone,
        T: Copy,
    {
        let mut remaining = self.clone();
        let mut divided: Vec<Self> = Vec::new();
        for v in values {
            let (divided1, divided2) = remaining.divide(*v, axis);
            divided.push(divided1);
            remaining = divided2;
        }
        divided.push(remaining.clone());
        divided
    }

    /// dividing a rectangle into specified weights of rectangles specified by axis
    fn divide_by_weights_and_axis(&self, weights: &Vec<T>, axis: Axis) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T>,
        T: Copy
            + std::ops::Add<Output = T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>,
    {
        if weights.is_empty() {
            return vec![];
        }
        if weights.len() == 1 {
            return vec![self.clone()];
        }
        let sum_of_weights: T = weights.iter().sum();
        let size = self.size_for_axis(axis);
        let values: Vec<T> = weights.iter().map(|w| *w * size / sum_of_weights).collect();
        // last value is not used
        let values = values[0..values.len() - 1].to_vec();
        self.divide_by_values_and_axis(&values, axis)
    }

    fn divide_vertical_then_horizontal_with_weights(
        &self,
        weights: &[T],
        aspect_ratio: T,
    ) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T> + Area<T>,
        T: Copy
            + std::ops::Add<Output = T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + std::cmp::PartialOrd,
    {
        let total_weight: T = weights.iter().sum();
        let total_area: T = self.area();
        let height = self.height();

        let mut dividing_weights: Vec<Vec<T>> = Vec::new();

        let mut remaining_weights = Vec::from(weights);
        let mut picked_weights: Vec<T> = Vec::new();
        let mut divided: Vec<Self> = Vec::new();

        // pick weights until the aspect ratio is satisfied
        while let Some(picked_weight) = remaining_weights.pop() {
            picked_weights.push(picked_weight);
            let total_picked_weight: T = picked_weights.iter().sum();
            let picked_area: T = total_picked_weight * total_area / total_weight;
            let picked_width = picked_area / height;
            let first_rect_weight = picked_weights[0];
            let first_rect_height = height * first_rect_weight / total_picked_weight;
            let first_rect_aspect_ratio = picked_width / first_rect_height;
            if first_rect_aspect_ratio >= aspect_ratio {
                // aspect ratio is satisfied (or over)
                dividing_weights.push(picked_weights);
                picked_weights = Vec::new();
            }
        }
        if !picked_weights.is_empty() {
            dividing_weights.push(picked_weights.clone());
        }

        let group_weights: Vec<T> = dividing_weights.iter().map(|w| w.iter().sum()).collect();

        let vertical_divided = self.divide_by_weights_and_axis(&group_weights, Axis::Vertical);
        for (divided_part, weights) in vertical_divided.iter().zip(dividing_weights.iter()) {
            let horizontal_divided =
                divided_part.divide_by_weights_and_axis(weights, Axis::Horizontal);
            divided.extend(horizontal_divided)
        }

        divided
    }

    fn divide_horizontal_then_vertical_with_weights(
        &self,
        weights: &[T],
        aspect_ratio: T,
    ) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T> + Area<T> + QuarterRotation,
        T: Copy
            + std::ops::Add<Output = T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>
            + std::cmp::PartialOrd,
    {
        // rotate, divide vertical, rotate back again means divide horizontal
        let rotated = self.rotate_clockwise();
        let divided = rotated.divide_vertical_then_horizontal_with_weights(weights, aspect_ratio);
        divided
            .iter()
            .map(|r| r.rotate_counter_clockwise())
            .collect()
    }
}

pub(crate) trait VerticalDividingHelper<T> {
    fn divide_vertical_helper(&self, x: T) -> (Self, Self)
    where
        Self: Sized;
}

impl<T, U> Dividing<T> for U
where
    U: QuarterRotation + VerticalDividingHelper<T>,
    T: Copy,
{
    fn divide_vertical(&self, x: T) -> (Self, Self) {
        self.divide_vertical_helper(x)
    }

    fn divide_horizontal(&self, y: T) -> (Self, Self) {
        // rotate, divide vertical, rotate back again means divide horizontal
        let rotated = self.rotate_clockwise();
        let (a, b) = rotated.divide_vertical(y);
        (a.rotate_counter_clockwise(), b.rotate_counter_clockwise())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis_aligned_rectangle::AxisAlignedRectangle;
    use crate::component::Component;
    use crate::point::Point;
    use crate::rectangle::Rectangle;
    use crate::weight::normalize_weights;

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
        assert_no_overlaps(&a_rect, &divided);
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
        assert_no_overlaps(&a_rect, &divided);
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
    fn test_divide_vertical_then_horizontal_with_weights() {
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[1], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[2], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[3], Rectangle::new(50.0, 50.0));

        // not divided case
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], rect);

        let rect = AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(100.0, 100.0));
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(
            divided[0],
            AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[1],
            AxisAlignedRectangle::new(Point::new(0.0, 50.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[2],
            AxisAlignedRectangle::new(Point::new(50.0, 0.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[3],
            AxisAlignedRectangle::new(Point::new(50.0, 50.0), Rectangle::new(50.0, 50.0))
        );

        // not divided case
        let rect = AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(100.0, 100.0));
        let weights = vec![1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(divided[0], rect);
    }

    #[test]
    fn test_divide_horizontal_then_vertical_with_weights() {
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[1], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[2], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[3], Rectangle::new(50.0, 50.0));

        // not divided case
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], rect);

        let rect = AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(100.0, 100.0));
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(
            divided[0],
            AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[1],
            AxisAlignedRectangle::new(Point::new(50.0, 0.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[2],
            AxisAlignedRectangle::new(Point::new(0.0, 50.0), Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[3],
            AxisAlignedRectangle::new(Point::new(50.0, 50.0), Rectangle::new(50.0, 50.0))
        );
    }

    fn assert_weights_dividing<T, D>(original: &D, divided: &[D], weights: &[T])
    where
        D: Dividing<T> + Area<T>,
        T: Copy
            + std::fmt::Debug
            + std::cmp::PartialEq
            + std::ops::Add<Output = T>
            + std::iter::Sum<T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>,
    {
        // check that the number of divided rectangles is equal to the number of weights
        assert_eq!(divided.len(), weights.len());

        // check that the sum of divided areas is equal to the original area
        let original_area = original.area();
        let divided_area: T = divided.iter().map(|r| r.area()).sum();
        assert_eq!(original_area, divided_area);

        // check that the sum of divided weights is equal to the original weight
        let original_normalized_weights = normalize_weights(weights);
        let divided_areas: Vec<T> = divided.iter().map(|r| r.area()).collect();
        let divided_area_by_weights = normalize_weights(&divided_areas);
        assert_eq!(original_normalized_weights, divided_area_by_weights);
    }

    fn assert_no_overlaps<T>(
        original: &AxisAlignedRectangle<T>,
        divided: &[AxisAlignedRectangle<T>],
    ) where
        T: Copy
            + std::fmt::Debug
            + std::cmp::PartialEq
            + std::ops::Add<Output = T>
            + std::iter::Sum<T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::cmp::PartialOrd,
    {
        // check all divided rectangles are inside the original rectangle
        for d in divided {
            assert!(original.enclodes(d));
        }
        // check no overlap between divided rectangles
        for (d1, d2) in divided.iter().zip(divided.iter().skip(1)) {
            assert!(!d1.overlaps(d2));
        }
    }
}
