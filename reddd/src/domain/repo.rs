use chrono::{DateTime, Utc};

use super::{Entity, MutableEntity};

#[derive(Clone, Copy, Debug)]
pub struct Pagination<'a, E: Entity> {
    pub before_key: &'a E::Key,
    pub before_timestamp: &'a DateTime<Utc>,
    pub page_size: usize,
}

#[async_trait::async_trait]
pub trait Repo {
    type Entity: Entity;

    async fn get(
        &self,
        key: &<Self::Entity as Entity>::Key,
    ) -> error::RepoResult<Self::Entity>;

    async fn get_page<'a>(
        &self,
        params: Pagination<'a, Self::Entity>,
    ) -> error::RepoResult<Vec<Self::Entity>>;

    async fn exists(
        &self,
        key: &<Self::Entity as Entity>::Key,
    ) -> error::RepoResult<bool>;
}


pub mod error {
    use std::error::Error;

    use thiserror::Error;

    pub type RepoResult<T> = Result<T, RepoError>;

    #[derive(Debug, Error)]
    pub enum RepoError {
        #[error("i/o error: {0}")]
        Io(std::io::Error),

        #[error("not found: {0}")]
        NotFound(String),

        #[error("duplicate item: {0}")]
        DuplicateValue(String),

        #[error("invlalid parameter: {0}")]
        InvalidParameter(String),

        #[error(transparent)]
        Other(Box<dyn Error>),
    }
}
