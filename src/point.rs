use crate::vector::Vector;

/// A point in 2D space
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// A point in 2D space constructor
impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

/// A point in 2D space with default values. in many cases, this is (0, 0)
impl<T: Default> Point<T> {
    pub fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

/// Vector from point A to point B
impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Point::new(2, 2);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_default() {
        let result = Point::<i32>::default();
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn test_sub() {
        let a = Point::new(2, 2);
        let b = Point::new(1, 1);
        let result = a - b;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 1);
    }
}
