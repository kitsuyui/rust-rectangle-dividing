use num_traits::{Num, NumAssignOps, NumOps};

use crate::{
    area::Area,
    axis::{Axis, SizeForAxis},
    rectangle::RectangleSize,
    rotate::QuarterRotation,
    weight::normalize_weights,
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
        T: Copy + Num + NumAssignOps,
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
    fn divide_by_weights_and_axis(&self, weights: &[T], axis: Axis) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T>,
        T: Copy + for<'a> std::iter::Sum<&'a T> + Num + NumAssignOps + NumOps,
    {
        if weights.is_empty() {
            return vec![];
        }
        if weights.len() == 1 {
            return vec![self.clone()];
        }
        let normalized_weights_ = normalize_weights(weights);
        let size: T = self.size_for_axis(axis);
        let mut values: Vec<T> = normalized_weights_.iter().map(|w| *w * size).collect();
        // last value is not used
        values.pop();
        self.divide_by_values_and_axis(&values, axis)
    }

    fn divide_vertical_then_horizontal_with_weights(
        &self,
        weights: &[T],
        aspect_ratio: T,
        boustrophedon: bool,
    ) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T> + Area<T>,
        T: Copy + for<'a> std::iter::Sum<&'a T> + Num + NumAssignOps + std::cmp::PartialOrd,
    {
        let norm_weights = normalize_weights(weights);
        let total_area = self.area();
        let height = self.height();

        let mut dividing_weights: Vec<Vec<T>> = Vec::new();

        let mut remaining_weights = norm_weights;
        let mut picked_weights: Vec<T> = Vec::new();
        let mut divided: Vec<Self> = Vec::new();

        remaining_weights.reverse(); // pop() removes item from the end of the vector, so reverse it
        // pick weights until the aspect ratio is satisfied
        while let Some(picked_weight) = remaining_weights.pop() {
            picked_weights.push(picked_weight);
            let weights_in_group = picked_weights.iter().sum::<T>();
            let picked_area: T = total_area * weights_in_group;
            let width = picked_area / height;
            let first_item_height = picked_weights[0] / weights_in_group * height;
            let first_item_aspect_ratio = width / first_item_height;
            if first_item_aspect_ratio >= aspect_ratio {
                dividing_weights.push(picked_weights.clone());
                picked_weights = Vec::new();
            }
        }
        if !picked_weights.is_empty() {
            dividing_weights.push(picked_weights.clone());
        }

        let group_weights: Vec<T> = dividing_weights.iter().map(|w| w.iter().sum()).collect();
        let vertical_divided = self.divide_by_weights_and_axis(&group_weights, Axis::Vertical);
        let mut forward = true;
        for (divided_part, weights) in vertical_divided.iter().zip(dividing_weights.iter_mut()) {
            if !forward {
                weights.reverse();
            }
            let mut horizontal_divided =
                divided_part.divide_by_weights_and_axis(weights, Axis::Horizontal);
            if !forward {
                horizontal_divided.reverse();
            }
            divided.extend(horizontal_divided);
            if boustrophedon {
                forward = !forward;
            }
        }
        divided
    }

    fn divide_horizontal_then_vertical_with_weights(
        &self,
        weights: &[T],
        aspect_ratio: T,
        boustrophedon: bool,
    ) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T> + Area<T> + QuarterRotation,
        T: Copy
            + Num
            + NumOps
            + NumAssignOps
            + std::cmp::PartialOrd
            + for<'a> std::iter::Sum<&'a T>,
    {
        // rotate, divide vertical, rotate back again means divide horizontal
        let rotated = self.rotate_clockwise();
        let rotated_aspect_ratio = T::one() / aspect_ratio;
        let divided = rotated.divide_vertical_then_horizontal_with_weights(
            weights,
            rotated_aspect_ratio,
            boustrophedon,
        );
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
    use num_traits::Float;

    use super::*;
    use crate::aspect_ratio::AspectRatio;
    use crate::axis_aligned_rectangle::AxisAlignedRectangle;
    use crate::component::Component;
    use crate::point::Point;
    use crate::rectangle::Rectangle;
    use crate::weight::normalize_weights;

    #[test]
    fn test_divide_vertical() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(&point, &rect).divide_vertical(2);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(2, 5));
        assert_eq!(rect_b.origin(), Point::new(4, 3));
        assert_eq!(rect_b.rect(), Rectangle::new(2, 5));

        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(&point, &rect).divide_vertical(1);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(1, 5));
        assert_eq!(rect_b.origin(), Point::new(3, 3));
        assert_eq!(rect_b.rect(), Rectangle::new(3, 5));
    }

    #[test]
    fn test_divide_horizontal() {
        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(&point, &rect).divide_horizontal(1);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(4, 1));
        assert_eq!(rect_b.origin(), Point::new(2, 4));
        assert_eq!(rect_b.rect(), Rectangle::new(4, 4));

        let point = Point::new(2, 3);
        let rect = Rectangle::new(4, 5);
        let (rect_a, rect_b) = AxisAlignedRectangle::new(&point, &rect).divide_horizontal(2);
        assert_eq!(rect_a.origin(), point);
        assert_eq!(rect_a.rect(), Rectangle::new(4, 2));
        assert_eq!(rect_b.origin(), Point::new(2, 5));
        assert_eq!(rect_b.rect(), Rectangle::new(4, 3));
    }

    #[test]
    fn test_divide_nth() {
        // test vertical
        let point = Point::new(2.0, 3.0);
        let rect = Rectangle::new(6.0, 2.0);
        let a_rect = AxisAlignedRectangle::new(&point, &rect);
        let divided = a_rect.divide_by_values_and_axis(&vec![1.0, 2.0], Axis::Vertical);
        assert_eq!(divided[0].origin(), point);
        assert_eq!(divided[0].rect(), Rectangle::new(1.0, 2.0));
        assert_eq!(divided[1].origin(), Point::new(3.0, 3.0));
        assert_eq!(divided[1].rect(), Rectangle::new(2.0, 2.0));
        assert_eq!(divided[2].origin(), Point::new(5.0, 3.0));
        assert_eq!(divided[2].rect(), Rectangle::new(3.0, 2.0));
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
        let point = Point::new(2.0, 3.0);
        let rect = Rectangle::new(2.0, 6.0);
        let a_rect = AxisAlignedRectangle::new(&point, &rect);
        let divided = a_rect.divide_by_values_and_axis(&vec![3.0, 2.0], Axis::Horizontal);
        assert_eq!(divided[0].origin(), point);
        assert_eq!(divided[0].rect(), Rectangle::new(2.0, 3.0));
        assert_eq!(divided[1].origin(), Point::new(2.0, 6.0));
        assert_eq!(divided[1].rect(), Rectangle::new(2.0, 2.0));
        assert_eq!(divided[2].origin(), Point::new(2.0, 8.0));
        assert_eq!(divided[2].rect(), Rectangle::new(2.0, 1.0));
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
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[1], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[2], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[3], Rectangle::new(50.0, 50.0));

        // not divided case
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], rect);

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(100.0, 100.0));
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_respect_aspect_ratio(&divided, &weights, 1.0);
        assert_eq!(
            divided[0],
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[1],
            AxisAlignedRectangle::new(&Point::new(0.0, 50.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[2],
            AxisAlignedRectangle::new(&Point::new(50.0, 0.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[3],
            AxisAlignedRectangle::new(&Point::new(50.0, 50.0), &Rectangle::new(50.0, 50.0))
        );

        // not divided case
        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(100.0, 100.0));
        let weights = vec![1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(divided[0], rect);

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(9.0, 8.0));
        let weights = vec![4.0, 4.0, 1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.5, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_respect_aspect_ratio(&divided, &weights, 1.5);
        assert_eq!(
            divided[0].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(6.0, 4.0))
        );
        assert_eq!(
            divided[1].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 4.0), &Rectangle::new(6.0, 4.0))
        );
        assert_eq!(
            divided[2].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 0.0), &Rectangle::new(3.0, 2.0))
        );
        assert_eq!(
            divided[3].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 2.0), &Rectangle::new(3.0, 2.0))
        );
        assert_eq!(
            divided[4].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 4.0), &Rectangle::new(3.0, 2.0))
        );
        assert_eq!(
            divided[5].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 6.0), &Rectangle::new(3.0, 2.0))
        );

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(300.0, 200.0));
        let weights = vec![4.0, 3.0, 2.0, 1.0];
        let divided = rect.divide_vertical_then_horizontal_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(
            divided[0].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(210.0, 114.0))
        );
        assert_eq!(
            divided[1].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 115.0), &Rectangle::new(210.0, 85.0))
        );
        assert_eq!(
            divided[2].round(),
            AxisAlignedRectangle::new(&Point::new(210.0, 0.0), &Rectangle::new(90.0, 133.0))
        );
        assert_eq!(
            divided[3].round(),
            AxisAlignedRectangle::new(&Point::new(210.0, 134.0), &Rectangle::new(90.0, 66.0))
        );
    }

    #[test]
    fn test_divide_horizontal_then_vertical_with_weights() {
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[1], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[2], Rectangle::new(50.0, 50.0));
        assert_eq!(divided[3], Rectangle::new(50.0, 50.0));

        // not divided case
        let rect = Rectangle::new(100.0, 100.0);
        let weights = vec![1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_eq!(divided[0], rect);

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(100.0, 100.0));
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_respect_aspect_ratio(&divided, &weights, 1.0);
        assert_eq!(
            divided[0],
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[1],
            AxisAlignedRectangle::new(&Point::new(50.0, 0.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[2],
            AxisAlignedRectangle::new(&Point::new(0.0, 50.0), &Rectangle::new(50.0, 50.0))
        );
        assert_eq!(
            divided[3],
            AxisAlignedRectangle::new(&Point::new(50.0, 50.0), &Rectangle::new(50.0, 50.0))
        );

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(8.0, 9.0));
        let weights = vec![4.0, 4.0, 1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0 / 1.5, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_respect_aspect_ratio(&divided, &weights, 1.0 / 1.5);
        assert_eq!(
            divided[0].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(4.0, 6.0))
        );
        assert_eq!(
            divided[1].round(),
            AxisAlignedRectangle::new(&Point::new(4.0, 0.0), &Rectangle::new(4.0, 6.0))
        );
        assert_eq!(
            divided[2].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[3].round(),
            AxisAlignedRectangle::new(&Point::new(2.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[4].round(),
            AxisAlignedRectangle::new(&Point::new(4.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[5].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 6.0), &Rectangle::new(2.0, 3.0))
        );

        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(300.0, 200.0));
        let weights = vec![4.0, 3.0, 2.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0, false);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_eq!(
            divided[0].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(133.0, 180.0))
        );
        assert_eq!(
            divided[1].round(),
            AxisAlignedRectangle::new(&Point::new(134.0, 0.0), &Rectangle::new(99.0, 180.0))
        );
        assert_eq!(
            divided[2].round(),
            AxisAlignedRectangle::new(&Point::new(234.0, 0.0), &Rectangle::new(66.0, 180.0))
        );
        assert_eq!(
            divided[3].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 180.0), &Rectangle::new(300.0, 20.0))
        );
    }

    #[test]
    fn test_divide_many() {
        // various pattern
        let rects = vec![
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(100.0, 100.0)),
            AxisAlignedRectangle::new(&Point::new(100.0, 0.0), &Rectangle::new(100.0, 300.0)),
            AxisAlignedRectangle::new(&Point::new(0.0, 100.0), &Rectangle::new(300.0, 100.0)),
            AxisAlignedRectangle::new(&Point::new(0.0, 100.0), &Rectangle::new(300.0, 300.0)),
        ];
        let various_weights = vec![vec![
            24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0,
            10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        ]];
        let various_aspect_ratio = vec![0.5, 1.0, 2.0];
        for rect in &rects {
            for weights in &various_weights {
                let divided = rect.divide_by_weights_and_axis(weights, Axis::Vertical);
                assert_weights_dividing(rect, &divided, weights);
                assert_no_overlaps(rect, &divided);
            }
        }
        for rect in &rects {
            for weights in &various_weights {
                for aspect_ratio in &various_aspect_ratio {
                    for boustrophedon in &[false, true] {
                        let divided = rect.divide_vertical_then_horizontal_with_weights(
                            weights,
                            *aspect_ratio,
                            *boustrophedon,
                        );
                        assert_respect_aspect_ratio(&divided, weights, *aspect_ratio);
                        assert_weights_dividing(rect, &divided, weights);
                        assert_no_overlaps(rect, &divided);
                        assert_respect_aspect_ratio(&divided, weights, rect.aspect_ratio());

                        let divided = rect.divide_horizontal_then_vertical_with_weights(
                            weights,
                            *aspect_ratio,
                            *boustrophedon,
                        );
                        assert_respect_aspect_ratio(&divided, weights, *aspect_ratio);
                        assert_weights_dividing(rect, &divided, weights);
                        assert_no_overlaps(rect, &divided);
                        assert_respect_aspect_ratio(&divided, weights, rect.aspect_ratio());
                    }
                }
            }
        }
    }

    #[test]
    fn test_boustrophedon() {
        let rect = AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(8.0, 9.0));
        let weights = vec![4.0, 4.0, 1.0, 1.0, 1.0, 1.0];
        let divided = rect.divide_horizontal_then_vertical_with_weights(&weights, 1.0 / 1.5, true);
        assert_weights_dividing(&rect, &divided, &weights);
        assert_no_overlaps(&rect, &divided);
        assert_respect_aspect_ratio(&divided, &weights, 1.0 / 1.5);
        assert_eq!(
            divided[0].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 0.0), &Rectangle::new(4.0, 6.0))
        );
        assert_eq!(
            divided[1].round(),
            AxisAlignedRectangle::new(&Point::new(4.0, 0.0), &Rectangle::new(4.0, 6.0))
        );
        assert_eq!(
            divided[2].round(),
            AxisAlignedRectangle::new(&Point::new(6.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[3].round(),
            AxisAlignedRectangle::new(&Point::new(4.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[4].round(),
            AxisAlignedRectangle::new(&Point::new(2.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
        assert_eq!(
            divided[5].round(),
            AxisAlignedRectangle::new(&Point::new(0.0, 6.0), &Rectangle::new(2.0, 3.0))
        );
    }

    fn assert_weights_dividing<T, D>(original: &D, divided: &[D], weights: &[T])
    where
        D: Dividing<T> + Area<T>,
        T: Copy
            + std::fmt::Debug
            + Num
            + NumAssignOps
            + NumOps
            + std::iter::Sum<T>
            + for<'a> std::iter::Sum<&'a T>
            + std::cmp::PartialOrd<f64>,
    {
        // check that the number of divided rectangles is equal to the number of weights
        assert_eq!(divided.len(), weights.len());

        // check that the sum of divided areas is equal to the original area
        let original_area = original.area();
        let divided_area: T = divided.iter().map(|r| r.area()).sum();
        // assert_eq!(original_area, divided_area);
        assert!((original_area - divided_area) < 0.1);

        // check that the sum of divided weights is equal to the original weight
        let original_normalized_weights = normalize_weights(weights);
        let divided_areas: Vec<T> = divided.iter().map(|r| r.area()).collect();
        let divided_area_by_weights = normalize_weights(&divided_areas);

        // assert_eq!(original_normalized_weights, divided_area_by_weights);
        let diffs: Vec<T> = original_normalized_weights
            .iter()
            .zip(divided_area_by_weights.iter())
            .map(|(w1, w2)| (*w1 - *w2) * (*w1 - *w2))
            .collect();

        for diff in diffs {
            assert!(diff < 0.1);
        }

        // assert_eq!(original_normalized_weights, divided_area_by_weights);
    }

    fn assert_no_overlaps<T>(
        original: &AxisAlignedRectangle<T>,
        divided: &[AxisAlignedRectangle<T>],
    ) where
        T: Copy
            + std::fmt::Debug
            + Num
            + NumAssignOps
            + NumOps
            + Float
            + std::iter::Sum<T>
            + for<'a> std::iter::Sum<&'a T>,
    {
        // check all divided rectangles are inside the original rectangle
        for d in divided {
            assert!(original.enclodes(&d.round()));
        }
        // check no overlap between divided rectangles
        for (d1, d2) in divided.iter().zip(divided.iter().skip(1)) {
            assert!(!d1.round().overlaps(&d2.round()));
        }
    }

    fn assert_respect_aspect_ratio<T>(
        divided: &[AxisAlignedRectangle<T>],
        weights: &[T],
        aspect_ratio: T,
    ) where
        T: Copy
            + std::fmt::Debug
            + std::cmp::PartialEq
            + for<'a> std::iter::Sum<&'a T>
            + Num
            + NumAssignOps
            + NumOps
            + Float
            + std::cmp::PartialOrd<f64>,
    {
        let normalized_weights = normalize_weights(weights);
        for (d, w) in divided.iter().zip(normalized_weights.iter()) {
            let asis_aspect_ratio = d.aspect_ratio();
            let diff = (asis_aspect_ratio - aspect_ratio).abs();
            // ideal diff must be 1.0 (same aspect ratio) but the real diff is not 1.0
            // assert that the diff is not too big, not too small
            // larger weights are expected to the diff must be smaller, smaller weights are expected to the diff must be smaller
            assert!(diff * *w < 0.5);
        }
    }
}
