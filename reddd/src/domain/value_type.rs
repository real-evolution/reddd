pub trait ValueType {
    type Value;

    fn value(self) -> Self::Value;
    fn value_ref(&self) -> &Self::Value;
}
