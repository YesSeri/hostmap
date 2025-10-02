pub fn log_entry_bulk() -> &'static str {
    "/api/log_entry/bulk"
}
pub fn hosts_bulk() -> &'static str {
    "/api/hosts/bulk"
}
pub fn frontpage() -> &'static str {
    "/"
}
pub fn history_page(hostname: &str) -> String {
    "/".to_string() + hostname
}

pub fn mapping_entry() -> &'static str {
    "/api/mapping"
}
pub fn mapping_entry_bulk() -> &'static str {
    "/api/mapping/bulk"
}
