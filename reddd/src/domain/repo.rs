use super::Entity;

#[async_trait::async_trait]
pub trait Repo {
    type Entity: Entity;
}
}
