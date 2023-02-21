use chrono::{DateTime, Utc};

use super::{Entity, MutableEntity};

/// A struct that holds repository pagination info.
///
/// # Example
///
/// ```ignore
/// let my_user = users_repo.get(&user_id).await?;
///
/// // get 10 items before `my_user`
/// let prev_page = users_repo.get_page(Pagiation {
///     before_key: my_user.id(),
///     before_timestamp: my_user.created_at(),
///     page_size: 10,
/// }).await?;
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Pagination<'a, E: Entity> {
    /// The key which to get items before.
    ///
    /// This field is needed to eliminate possible duplications that can
    /// occure when paginating using timestamp only, as multiple records
    /// can have the same timestamp.
    pub before_key: &'a E::Key,

    /// The creation timestamp which to get items before.
    ///
    /// This is usually obtained using [created_at](Entity::created_at())
    /// method of an entity.
    pub before_timestamp: &'a DateTime<Utc>,

    /// The number of items to include in a page.
    pub page_size: usize,
}

/// A trait to be implemented by data repositories.
///
/// This trait provides read-only methods that have no side effects on the
/// stored data.
#[async_trait::async_trait]
pub trait Repo {
    /// The type of the entity that this repository operates on.
    type Entity: Entity;

    /// Gets a single item by its identifier.
    ///
    /// # Arguments
    ///
    /// * `key` - The identifier of the item to get.
    async fn get(
        &self,
        key: &<Self::Entity as Entity>::Key,
    ) -> error::RepoResult<Self::Entity>;

    /// Gets a page of items using passed pagination parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Pagination parameters to get the page with.
    async fn get_page<'a>(
        &self,
        params: Pagination<'a, Self::Entity>,
    ) -> error::RepoResult<Vec<Self::Entity>>;

    /// Checks whether an item with the passed identifier exists or not.
    ///
    /// # Arguments
    ///
    /// * `key` - The identifier of the item to check.
    async fn exists(
        &self,
        key: &<Self::Entity as Entity>::Key,
    ) -> error::RepoResult<bool>;
}

/// A trait to add mutability operations to repositories.
///
/// Unlike [`Repo`] trait, implementations of this trait can have side effects
/// on the stored data, thus extra care needs to be taken when used.
#[async_trait::async_trait]
pub trait MutableRepo {
    /// The type of the entity that this repository operates on.
    type Entity: MutableEntity;

    /// Adds an item to the data repository.
    ///
    /// # Arguments
    ///
    /// * `item` - Item to be added
    async fn add(&self, item: Self::Entity) -> error::RepoResult<Self::Entity>;

    /// Updates an item in the data repository.
    ///
    /// The item **must** be present in the data repository, as implementations
    /// of this trait **shall** not create the item if it does not exist.
    ///
    /// # Arguments
    ///
    /// * `item` - Item to be updated
    async fn update(
        &self,
        item: Self::Entity,
    ) -> error::RepoResult<Self::Entity>;

    /// Removes an item from the data repository.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the item to be removed
    async fn remove(
        &self,
        key: &<Self::Entity as Entity>::Key,
    ) -> error::RepoResult<()>;
}

pub mod error {
    use std::error::Error;

    use thiserror::Error;

    /// A type definition that simplifies [`Result<T, E>`] usage when dealing
    /// with data repositories.
    pub type RepoResult<T> = Result<T, RepoError>;

    /// An enumeration of possible errors that can occure during a repository
    /// operation execution.
    #[derive(Debug, Error)]
    pub enum RepoError {
        /// Internal Input/Output error.
        #[error("i/o error: {0}")]
        Io(std::io::Error),

        /// An item that was referred to does not exist.
        #[error("not found: {0}")]
        NotFound(String),

        /// An item with similar data to the item being added/updated already
        /// exists.
        #[error("duplicate item: {0}")]
        DuplicateValue(String),

        /// Invalid data was provided.
        #[error("invlalid parameter: {0}")]
        InvalidParameter(String),

        /// Other error types. This can hold user-defined types, as well as,
        /// any other error type.
        #[error(transparent)]
        Other(Box<dyn Error>),
    }
}
