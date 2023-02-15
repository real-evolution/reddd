use std::marker::PhantomData;

use reddd_macros::ValueType;

use crate::domain::ValueType;

/// A value of type `V` wrapper that is typed to a specific type `T`
///
/// Two types with the same `V` types are compatible if used with a function
/// that requires a [`ValueType<Value = V>`]. However, they **are not** when
/// used with functions that requires either concretely.
///
/// # Generic Parameters
///
/// * `V`: Type of wrapped values
/// * `T`: Type to distinguish wrappers with same `V` with
///
/// # Example
/// ```rust
/// type MyTypedValue1 = TypedValue<usize, MyFirstType>;
/// type MyTypedValue2 = TypedValue<usize, MySecondType>;
///
/// // `MyTypedValue1` and `MyTypedValue2` can be used here
/// fn generic_func<V: ValueType<Value = usize>>(value: V) {
///     // --snip--
/// }
///
/// // Only `MyTypedValue1` can be used here
/// fn concrete_func(value: MyTypedValue1) {
///     // --snip--
/// }
/// ````
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
