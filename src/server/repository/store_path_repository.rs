use sqlx::{Postgres, QueryBuilder};

use crate::server::RetError;

#[derive(Debug, Clone)]
pub struct StorePathRepository;

impl StorePathRepository {
    pub(crate) async fn bulk_insert_store_paths(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _recs: &[&str],
    ) -> Result<u64, RetError> {
        const CHUNK_SIZE: usize = 1000;
        let mut i = 0;
        for chunk in _recs.chunks(CHUNK_SIZE) {
            let mut query_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO nix_store_path(store_path) ");
            query_builder.push_values(chunk.iter(), |mut b, rec| {
                b.push_bind(rec.to_string());
            });
            // on conflict do nothing to avoid duplicate entries
            query_builder.push(" ON CONFLICT (store_path) DO NOTHING");
            let query = query_builder.build();
            let res = query.execute(&mut **tx).await?;
            i += res.rows_affected();
        }
        Ok(i)
    }
}
