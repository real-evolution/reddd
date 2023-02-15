use std::marker::PhantomData;

use reddd_macros::ValueType;

use crate::domain::ValueType;

#[derive(Debug, ValueType)]
pub struct TypedValue<V, T>(#[main_field] V, PhantomData<dyn Fn() -> T>)
where
    V: Clone + std::fmt::Debug + PartialEq + PartialOrd;

impl<V, T> Clone for TypedValue<V, T>
where
    V: Clone + std::fmt::Debug + PartialEq + PartialOrd,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}
