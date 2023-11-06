use crate::{
    axis::{Axis, SizeForAxis},
    rectangle::{Area, RectangleSize},
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
        if weights.len() <= 1 {
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
    use crate::point::Point;
    use crate::rectangle::Rectangle;
    use crate::weight::normalize_weights;

    #[test]
    fn test_divide_vertical_then_horizontal_with_weights() {
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided.len(), 4);
        assert_eq!(divided[0], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[1], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[2], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[3], Rectangle::new(50.0, 50.0));

        let rect = AxisAlignedRectangle::new(Point::new(0.0, 0.0), Rectangle::new(100.0, 100.0));
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0);
        assert_eq!(divided.len(), 4);
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
}
