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
