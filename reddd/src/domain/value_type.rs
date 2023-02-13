pub trait ValueType<T> {
    fn value(self) -> T;
    fn value_ref(&self) -> &T;
}
