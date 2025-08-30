#[derive(Debug, sqlx::FromRow)]
pub struct HostRowModel {
    pub host: String,
    pub host_url: String,
    pub loc: String,
    pub system: String,
    pub rev: String,
    pub rev_url: String,
    pub ref_: String,
}