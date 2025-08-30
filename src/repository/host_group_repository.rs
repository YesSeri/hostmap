use crate::{
    controller::frontpage::FilterParams,
    model::{host_group::HostGroupModel, host_row::HostRowModel},
};
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;

#[derive(sqlx::FromRow)]
struct HostGroupHostRow {
    host_group_id: i32,
    host_group_name: String,
    host: String,
    host_url: String,
    loc: String,
    system: String,
    rev: String,
    rev_url: String,
    ref_: String,
}

fn build_host_group_query(filter_params: &FilterParams) -> QueryBuilder<'_,Postgres> {
    let mut qb = QueryBuilder::new(
        r#"
        SELECT
            hg.id as host_group_id,
            hg.name as host_group_name,
            h.host, h.host_url, h.loc, h.system, h.rev, h.rev_url, h.ref_
        FROM host_groups hg
        INNER JOIN hosts h ON h.host_group_id = hg.id
        WHERE 1=1
        "#,
    );
    if !filter_params.hosts.is_empty() {
        qb.push(" AND hg.name IN (");
        let mut separated = qb.separated(", ");
        for host in &filter_params.hosts {
            separated.push_bind(host);
        }
        qb.push(")");
    }
    if let Some(search) = &filter_params.search {
        qb.push(" AND (");
        qb.push(" hg.name ILIKE ");
        qb.push_bind(format!("%{}%", search));
        qb.push(" OR ");
        qb.push(" h.host ILIKE ");
        qb.push_bind(format!("%{}%", search));
        qb.push(" OR ");
        qb.push(" h.system ILIKE ");
        qb.push_bind(format!("%{}%", search));
        qb.push(" OR ");
        qb.push(" h.rev ILIKE ");
        qb.push_bind(format!("%{}%", search));
        qb.push(")");
    }
    qb.push(" ORDER BY hg.name, h.host");
    qb
}

pub(crate) async fn fetch_all_host_groups_by_filter_params(
    db: &PgPool,
    filter_params: &FilterParams,
) -> Result<Vec<HostGroupModel>, sqlx::Error> {
    let filter_params_is_empty =
        filter_params.hosts.is_empty() && filter_params.search.as_deref().unwrap_or("").is_empty();
    if filter_params_is_empty {
        fetch_all_host_groups(db).await
    } else {
        let mut qb = build_host_group_query(filter_params);
        let query = qb.build_query_as::<HostGroupHostRow>();
        let rows = query.fetch_all(db).await?;
        Ok(set_relationship_host_group_host_row(rows))
    }
}

async fn fetch_all_host_groups(db: &PgPool) -> Result<Vec<HostGroupModel>, sqlx::Error> {
    let rows: Vec<HostGroupHostRow> = sqlx::query_as!(
        HostGroupHostRow,
        r#"
        SELECT
            hg.id as host_group_id,
            hg.name as host_group_name,
            h.host, h.host_url, h.loc, h.system, h.rev, h.rev_url, h.ref_ as "ref_"
        FROM host_groups hg
        INNER JOIN hosts h ON h.host_group_id = hg.id
        ORDER BY host_group_name, h.host
        "#
    )
    .fetch_all(db)
    .await?;
    let grouped = set_relationship_host_group_host_row(rows);
    Ok(grouped)
}

fn set_relationship_host_group_host_row(rows: Vec<HostGroupHostRow>) -> Vec<HostGroupModel> {
    let mut groups: HashMap<i32, HostGroupModel> = HashMap::new();

    for HostGroupHostRow {
        host_group_id,
        host_group_name,
        host,
        host_url,
        loc,
        system,
        rev,
        rev_url,
        ref_,
    } in rows
    {
        let group = groups
            .entry(host_group_id)
            .or_insert_with(|| HostGroupModel {
                id: host_group_id,
                name: host_group_name,
                rows: Vec::new(),
            });

        group.rows.push(HostRowModel {
            host,
            host_url,
            loc,
            system,
            rev,
            rev_url,
            ref_,
        });
    }

    let mut grouped: Vec<HostGroupModel> = groups.into_values().collect();
    grouped.sort_by(|a, b| a.name.cmp(&b.name));
    grouped
}

pub async fn fetch_all_host_group_names(db: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(r#"SELECT name FROM host_groups ORDER BY name"#)
        .fetch_all(db)
        .await?;
    let names: Vec<String> = rows.into_iter().map(|row| row.name).collect();
    Ok(names)
}
