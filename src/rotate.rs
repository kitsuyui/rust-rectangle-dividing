pub trait QuarterRotation
where
    Self: Sized,
{
    /// Rotate clockwise by 90 degrees
    fn rotate_clockwise(&self) -> Self;

    /// Rotate counter clockwise by 90 degrees
    fn rotate_counter_clockwise(&self) -> Self {
        // rotate clockwise 3 times is the same as rotate counter clockwise
        self.rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
    }
}
