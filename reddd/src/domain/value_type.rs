pub trait ValueType: Clone {
    type Value;

    fn value(self) -> Self::Value;
    fn value_ref(&self) -> &Self::Value;
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
}
