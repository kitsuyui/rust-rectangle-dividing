pub(crate) trait Component<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}
