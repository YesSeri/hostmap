use sqlx::{Postgres, QueryBuilder};

use crate::{server::RetError, shared::model::nix_git_link::NixGitLinkModel};

#[derive(Debug, Clone)]
pub struct NixGitLinkRepository;

impl NixGitLinkRepository {
    pub async fn create_many(
        tx: &mut sqlx::Transaction<'_, Postgres>,
        nix_git_links: Vec<NixGitLinkModel>,
    ) -> Result<u64, RetError> {
        const CHUNK_SIZE: usize = 1000;
        let mut i = 0;
        for chunk in nix_git_links.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO nix_git_link(store_path, commit_hash, branch, linked_at) ",
            );
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(&rec.nix_store_path)
                    .push_bind(&rec.revision.commit_hash)
                    .push_bind(&rec.revision.branch)
                    .push_bind(rec.linked_at);
            });
            query_builder.push(" ON CONFLICT (store_path, commit_hash, branch) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&mut **tx).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }
}
