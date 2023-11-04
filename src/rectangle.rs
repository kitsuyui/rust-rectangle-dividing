use crate::dividing::Dividing;
use crate::rotate::Rotate;
/// rectangle in 2D space with a width and height
pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

/// Area of an axis aligned rectangle
pub trait Area<T> {
    fn area(&self) -> T;
}

/// A rectangle in 2D space constructor
impl<T> Rectangle<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// Rotate a rectangle by 90 degrees
impl<T: Copy> Rotate for Rectangle<T> {
    fn rotate(&self) -> Self {
        Self {
            width: self.height,
            height: self.width,
        }
    }
}

/// Area of a rectangle
impl<T: std::ops::Mul<Output = T> + Copy> Rectangle<T> {
    pub fn area(&self) -> T {
        self.width * self.height
    }
}

impl<T: std::ops::Sub<Output = T> + Copy> Dividing<T> for Rectangle<T> {
    /// dividing a rectangle into two rectangles (vertical)
    fn divide_vertical(&self, x: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(x, self.height),
            Self::new(self.width - x, self.height),
        )
    }

    /// dividing a rectangle into two rectangles (horizontal)
    fn divide_horizontal(&self, y: T) -> (Rectangle<T>, Rectangle<T>) {
        (
            Self::new(self.width, y),
            Self::new(self.width, self.height - y),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Rectangle::new(2, 2);
        assert_eq!(result.width, 2);
        assert_eq!(result.height, 2);
    }

    #[test]
    fn test_rotate() {
        let result = Rectangle::new(2, 3).rotate();
        assert_eq!(result.width, 3);
        assert_eq!(result.height, 2);
    }

    #[test]
    fn test_area() {
        let result = Rectangle::new(2, 3).area();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_divide_vertical() {
        let (rect_a, rect_b) = Rectangle::new(4, 2).divide_vertical(1);
        assert_eq!(rect_a.width, 1);
        assert_eq!(rect_a.height, 2);
        assert_eq!(rect_b.width, 3);
        assert_eq!(rect_b.height, 2);
    }

    #[test]
    fn test_divide_horizontal() {
        let (rect_a, rect_b) = Rectangle::new(2, 4).divide_horizontal(1);
        assert_eq!(rect_a.width, 2);
        assert_eq!(rect_a.height, 1);
        assert_eq!(rect_b.width, 2);
        assert_eq!(rect_b.height, 3);
    }
}
