use crate::rectangle::RectangleSize;

pub enum DividingDirection {
    Vertical,
    Horizontal,
}

pub trait Dividing<T> {
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (Self, Self)
    where
        Self: Sized;

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (Self, Self)
    where
        Self: Sized;

    /// dividing a rectangle into two rectangles specified by direction
    fn divide(&self, v: T, direction: &DividingDirection) -> (Self, Self)
    where
        Self: Sized,
    {
        match direction {
            DividingDirection::Vertical => self.divide_vertical(v),
            DividingDirection::Horizontal => self.divide_horizontal(v),
        }
    }

    /// dividing a rectangle into specified number of rectangles specified by direction
    fn divide_by_values(&self, values: Vec<T>, direction: &DividingDirection) -> Vec<Self>
    where
        Self: Sized + RectangleSize<T> + Clone,
        T: Copy,
    {
        let mut remaining = self.clone();
        let mut divided: Vec<Self> = Vec::new();
        for v in values {
            let (divided1, divided2) = remaining.divide(v, direction);
            divided.push(divided1);
            remaining = divided2;
        }
        divided.push(remaining.clone());
        divided
    }
}
