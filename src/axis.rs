/// Direction of a dividing line used to split a rectangle.
///
/// `Vertical` means a **vertical cutting line** (runs top-to-bottom), which
/// splits the rectangle along the X dimension — each resulting piece has a
/// different width while the height is unchanged.
///
/// `Horizontal` means a **horizontal cutting line** (runs left-to-right), which
/// splits the rectangle along the Y dimension — each resulting piece has a
/// different height while the width is unchanged.
///
/// Note: the names refer to the *orientation of the cutting line*, not the
/// coordinate axis. `Vertical` therefore corresponds to X/width values, and
/// `Horizontal` to Y/height values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// A vertical cutting line; operates on the X dimension (width).
    Vertical,
    /// A horizontal cutting line; operates on the Y dimension (height).
    Horizontal,
}

impl Axis {
    /// Get the opposite axis of the current axis (Vertical <-> Horizontal)
    pub fn opposite(&self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
        }
    }
}

/// Returns the coordinate value for the given [`Axis`].
///
/// - `Axis::Vertical`   → X coordinate (the vertical cutting line operates on X)
/// - `Axis::Horizontal` → Y coordinate (the horizontal cutting line operates on Y)
pub trait ValueForAxis<T> {
    fn value_for_axis(&self, axis: Axis) -> T;
}

/// Returns the size (width or height) for the given [`Axis`].
///
/// - `Axis::Vertical`   → width  (the vertical cutting line divides width)
/// - `Axis::Horizontal` → height (the horizontal cutting line divides height)
pub trait SizeForAxis<T> {
    fn size_for_axis(&self, axis: Axis) -> T;
}
