/// A simple 2D vector
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

/// A simple 2D vector constructor
impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }
}

/// A simple 2D vector with default values. in many cases, this is (0, 0)
impl<T: Default> Vector<T> {
    pub fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

/// Add vector A to vector B
impl<T: std::ops::Add<Output = T>> std::ops::Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Vector::new(2, 2);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_default() {
        let result = Vector::<i32>::default();
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
    }

    #[test]
    fn test_add() {
        let a = Vector::new(2, 2);
        let b = Vector::new(1, 1);
        let result = a + b;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
    }
}
