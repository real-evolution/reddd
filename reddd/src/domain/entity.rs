use chrono::{DateTime, Utc};

use super::TypedValue;

pub type Key<E, V> = TypedValue<V, E>;

/// A trait to be implemented by domain entities.
///
/// This trait is used to identify domain entities, and expose common fields
/// through getters.
pub trait Entity {
    /// The type of the key that is used to identify entities.
    type Key;

    /// Gets a reference to the key of the entity.
    fn id(&self) -> &Self::Key;

    /// Gets a reference to the timestamp at which the entity was created.
    fn created_at(&self) -> &DateTime<Utc>;
}

/// A trait that provides mutability support on top of [`Entity`] trait
///
/// The main difference between the traits [`MutableEntity`] and [`Entity`] is
/// that [`MutableEntity`] exposes the last modification timestamp.
pub trait MutableEntity: Entity {
    /// Gets the timestamp at which the entity was last updated
    fn updated_at(&self) -> &DateTime<Utc>;

    /// Updates the modification timestamp to [`chrono::Utc::now()`]
    fn touch(&mut self) -> &DateTime<Utc>;
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use fake::{Dummy, Fake, Faker};
    use reddd_macros::{Entity, MutableEntity};
    use uuid::Uuid;

    use super::*;

    #[cfg(test)]
    impl<E, T> Dummy<Faker> for Key<E, T>
    where
        E: Entity,
        T: Clone + std::fmt::Debug + Dummy<Faker> + PartialEq + PartialOrd,
    {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(
            config: &Faker,
            _rng: &mut R,
        ) -> Self {
            Self::new(config.fake())
        }
    }

    #[test]
    fn entity_auto_test() {
        #[derive(Entity, Dummy)]
        struct User {
            id: u32,
            created_at: DateTime<Utc>,
        }

        let user: User = Faker.fake();

        assert_eq!(user.id, *user.id());
        assert_eq!(user.created_at, *user.created_at());
    }

    #[test]
    fn entity_annotated_test() {
        #[derive(Entity, Dummy)]
        struct User {
            #[id_field]
            key_field: Uuid,

            #[created_at_field]
            registered_on: DateTime<Utc>,
        }

        let user: User = Faker.fake();

        assert_eq!(&user.key_field, user.id());
        assert_eq!(&user.registered_on, user.created_at());
    }

    #[test]
    fn mutable_entity_auto_test() {
        #[derive(Debug, MutableEntity, Dummy)]
        struct User {
            id: Key<User, Uuid>,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
        }

        let mut user: User = Faker.fake();

        assert_eq!(&user.id, user.id());
        assert_eq!(&user.created_at, user.created_at());
        assert_eq!(&user.updated_at, user.updated_at());

        let old_timestamp = *user.updated_at();
        let new_timestamp = *user.touch();

        assert_ne!(old_timestamp, new_timestamp);
        assert_ne!(&old_timestamp, user.updated_at());
        assert_eq!(&new_timestamp, user.updated_at());
    }

    #[test]
    fn mutable_entity_annotated_test() {
        #[derive(Debug, MutableEntity, Dummy)]
        struct User {
            #[id_field]
            key_field: Key<User, Uuid>,

            #[created_at_field]
            registered_on: DateTime<Utc>,

            #[updated_at_field]
            modified_at: DateTime<Utc>,
        }

        let mut user: User = Faker.fake();

        assert_eq!(&user.key_field, user.id());
        assert_eq!(&user.registered_on, user.created_at());
        assert_eq!(&user.modified_at, user.updated_at());

        let old_timestamp = *user.updated_at();
        let new_timestamp = *user.touch();

        assert_ne!(old_timestamp, new_timestamp);
        assert_ne!(&old_timestamp, user.updated_at());
        assert_eq!(&new_timestamp, user.updated_at());
    }
}
