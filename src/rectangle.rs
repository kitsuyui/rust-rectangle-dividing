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
        Rectangle { width, height }
    }
}

/// Rotate a rectangle by 90 degrees
impl<T: Copy> Rotate for Rectangle<T> {
    fn rotate(&self) -> Self {
        Rectangle {
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
}
