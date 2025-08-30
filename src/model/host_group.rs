use crate::model::host_row::HostRowModel;

#[derive(Debug, sqlx::FromRow)]
pub struct HostGroupModel {
    pub id: i32,
    pub name: String,
    pub rows: Vec<HostRowModel>,
}
