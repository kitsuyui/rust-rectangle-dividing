use crate::{
    axis::{Axis, SizeForAxis},
    rectangle::RectangleSize,
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
    fn divide(&self, v: T, axis: &Axis) -> (Self, Self)
    where
        Self: Sized,
    {
        match axis {
            Axis::Vertical => self.divide_vertical(v),
            Axis::Horizontal => self.divide_horizontal(v),
        }
    }

    /// dividing a rectangle into specified number of rectangles specified by axis
    fn divide_by_values(&self, values: Vec<T>, axis: &Axis) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone,
        T: Copy,
    {
        let mut remaining = self.clone();
        let mut divided: Vec<Self> = Vec::new();
        for v in values {
            let (divided1, divided2) = remaining.divide(v, axis);
            divided.push(divided1);
            remaining = divided2;
        }
        divided.push(remaining.clone());
        divided
    }

    /// dividing a rectangle into specified weights of rectangles specified by axis
    fn divide_by_weights(&self, weights: Vec<T>, axis: &Axis) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone + SizeForAxis<T>,
        T: Copy
            + std::ops::Add<Output = T>
            + for<'a> std::iter::Sum<&'a T>
            + std::ops::Div<Output = T>
            + std::ops::Mul<Output = T>,
    {
        let sum_of_weights: T = weights.iter().sum();
        let size = self.size_for_axis(axis);
        let values: Vec<T> = weights.iter().map(|w| *w * size / sum_of_weights).collect();
        // last value is not used
        let values = values[0..values.len() - 1].to_vec();
        self.divide_by_values(values, axis)
    }
}
