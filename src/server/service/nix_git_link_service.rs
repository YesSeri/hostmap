use sqlx::{Pool, Postgres};

use crate::{
    server::{
        RetError,
        repository::{
            nix_git_link_repository::NixGitLinkRepository,
            store_path_repository::StorePathRepository,
        },
    },
    shared::dto::nix_git_link::NixGitLinkDto,
};
#[derive(Debug, Clone)]
pub struct NixGitLinkService {
    pool: Pool<Postgres>,
}

impl NixGitLinkService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub async fn create_many(&self, nix_git_links: Vec<NixGitLinkDto>) -> Result<u64, RetError> {
        let mut tx = self.pool.begin().await?;
        let store_paths: Vec<&str> = nix_git_links
            .iter()
            .map(|el| el.nix_store_path.as_str())
            .collect();
        StorePathRepository::bulk_insert_store_paths(&mut tx, &store_paths).await?;
        let models = nix_git_links.into_iter().map(|dto| dto.into()).collect();
        let i = NixGitLinkRepository::create_many(&mut tx, models).await?;
        tx.commit().await?;
        Ok(i)
    }
    pub async fn create(&self, nix_git_link: NixGitLinkDto) -> Result<(), RetError> {
        self.create_many(vec![nix_git_link]).await?;
        Ok(())
    }
}
