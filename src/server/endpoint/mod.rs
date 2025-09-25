
pub fn log_entry_bulk() -> &'static str {
    "/api/log_entry/bulk"
}
pub fn hosts_bulk() -> &'static str {
    "/api/hosts/bulk"
}
pub fn frontpage() -> &'static str {
    "/"
}

pub fn assets_folder() -> &'static str {
    "/assets"
}

pub fn history_page(hostname: &str) -> String {
    "/".to_string() + hostname
}
