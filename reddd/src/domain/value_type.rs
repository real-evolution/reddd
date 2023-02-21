use std::marker::PhantomData;

use reddd_macros::ValueType;

/// A trait to be implemented by concrete value types.
///
/// This trait can be useful to "embed" domain rules (such as validation)
/// within the value type itself. It also can be used to strongly-type values
/// that could mean different things.
///
/// # Example
///
/// ```rust
/// use reddd::domain::ValueType;
///
/// const MIN_ALLOWED_BALANCE: f64 = 0.00;
/// const MAX_ALLOWED_BALANCE: f64 = 1_000_000.00;
///
/// enum BalanceError {
///     BalanceOutOfBounds,
///     // ...
/// }
///
/// #[derive(Clone)]
/// struct Balance(f64);
///
/// impl ValueType for Balance {
///     type Value = f64;
///
///     fn value(self) -> Self::Value {
///         self.0
///     }
///
///     fn value_ref(&self) -> &Self::Value {
///         &self.0
///     }
/// }
///
/// impl TryFrom<f64> for Balance {
///     type Error = BalanceError;
///
///     fn try_from(value: f64) -> Result<Self, Self::Error> {
///         if value < MIN_ALLOWED_BALANCE || value > MAX_ALLOWED_BALANCE {
///             return Err(BalanceError::BalanceOutOfBounds);
///         }
///
///         Ok(Self(value))
///     }
/// }
/// ```
pub trait ValueType: Clone {
    type Value: Clone + PartialEq + PartialOrd;

    /// Moves the wrapped value out of `self`
    fn value(self) -> Self::Value;

    /// Borrows a reference to the wrapped value
    fn value_ref(&self) -> &Self::Value;
}

/// A value of type `V` wrapper that is typed to a specific type `T`
///
/// Two types with the same `V` types are compatible if used with a function
/// that requires a [`ValueType<Value = V>`]. However, they **are not** when
/// used with functions that requires either concretely.
///
/// # Generic Arguments
///
/// * `V` - Type of wrapped values
/// * `T` - Type to distinguish wrappers with same `V` with
///
/// # Example
/// ```rust
/// use reddd::domain::{TypedValue, ValueType};
///
/// struct MyFirstType;
/// struct MySecondType;
///
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

impl<V, T> TypedValue<V, T>
where
    V: Clone + std::fmt::Debug + PartialEq + PartialOrd,
{
    pub fn new(value: V) -> Self {
        Self(value, Default::default())
    }
}

impl<V, T> Clone for TypedValue<V, T>
where
    V: Clone + std::fmt::Debug + PartialEq + PartialOrd,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

#[cfg(test)]
mod tests {
    use fake::{Dummy, Fake, Faker};
    use reddd_macros::ValueType;

    use super::*;

    #[test]
    fn simple_newtype_annotated_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType(#[main_field] i32);

        let f: NewType = Faker.fake();
        let fval = f.0;

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn simple_newtype_auto_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType(i32);

        let f: NewType = Faker.fake();
        let fval = f.0;

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn multifield_newtype_annotated_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType(i32, #[main_field] String);

        let f: NewType = Faker.fake();
        let fval = f.1.clone();

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn multifield_newtype_auto_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType(i32, String);

        let f: NewType = Faker.fake();
        let fval = f.0;

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn multifield_named_annotated_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType {
            _field0: i32,
            #[main_field]
            field1: String,
        }

        let f: NewType = Faker.fake();
        let fval = f.field1.clone();

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn multifield_named_auto_test() {
        #[derive(Clone, ValueType, Dummy)]
        struct NewType {
            field0: i32,
            _field1: String,
        }

        let f: NewType = Faker.fake();
        let fval = f.field0;

        assert_eq!(&fval, f.value_ref());
        assert_eq!(fval, f.value());
    }

    #[test]
    fn equlity_test() {
        #[derive(Clone, Debug, ValueType, Dummy)]
        struct NewType(String);

        let f1: NewType = Faker.fake();
        let mut f2 = f1.clone();

        assert_eq!(f1, f2);

        f2.0 = f1.0.clone() + "_additional"; // ensures not equal

        assert_ne!(f1, f2);
    }

    #[test]
    fn order_test() {
        #[derive(Clone, Debug, ValueType, Dummy)]
        struct NewType(u64);

        let mut f1: NewType = Faker.fake();
        let mut f2 = f1.clone();

        assert_eq!(f1, f2);

        assert!(f1 <= f2);
        assert!(f1 >= f2);
        assert!(f2 <= f1);
        assert!(f2 >= f1);

        while f1 == f2 {
            f1.0 += rand::random::<u64>() % (u64::MAX - f1.0);
        }

        assert_ne!(f1, f2);

        assert!(f1 > f2);
        assert!(f1 >= f2);
        assert!(f2 < f1);
        assert!(f2 <= f1);

        while f1 >= f2 {
            f2.0 += rand::random::<u64>() % (u64::MAX - f2.0);
        }

        assert_ne!(f1, f2);

        assert!(f1 < f2);
        assert!(f1 <= f2);
        assert!(f2 > f1);
        assert!(f2 >= f1);
    }
}
