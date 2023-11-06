pub(crate) trait QuarterRotation
where
    Self: Sized,
{
    fn rotate_clockwise(&self) -> Self;

    fn rotate_counter_clockwise(&self) -> Self {
        // rotate clockwise 3 times is the same as rotate counter clockwise
        self.rotate_clockwise()
            .rotate_clockwise()
            .rotate_clockwise()
    }
}
