use serde::Serialize;

use crate::{model::host_group::HostGroupModel, viewmodel::host_row::HostRowView};

#[derive(Debug, Serialize)]
pub struct HostGroupView {
    pub id: i32,
    pub name: String,
    pub rows: Vec<HostRowView>,
}

impl From<HostGroupModel> for HostGroupView {
    fn from(HostGroupModel { id, name, rows }: HostGroupModel) -> Self {
        let name = name.clone();
        let rows = rows.into_iter().map(HostRowView::from).collect();
        Self { id, name, rows }
    }
}
