pub(crate) trait AspectRatio<T> {
    /// Returns the aspect ratio (width / height) of the shape.
    #[allow(dead_code)]
    fn aspect_ratio(&self) -> T;
}
