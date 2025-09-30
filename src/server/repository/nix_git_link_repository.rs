use sqlx::{Pool, Postgres};

use crate::{server::RetError, shared::model::nix_git_link::NixGitLinkModel};

#[derive(Debug, Clone)]
pub struct NixGitLinkRepository {
    pool: Pool<Postgres>,
}

impl NixGitLinkRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub async fn create_many(&self, nix_git_links: Vec<NixGitLinkModel>) -> Result<u64, RetError> {
        todo!();
    }
    pub async fn create(&self, nix_git_link: NixGitLinkModel) -> Result<(), RetError> {
        todo!();
    }
}
