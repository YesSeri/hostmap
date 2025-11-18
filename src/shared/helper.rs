use std::path::PathBuf;

pub(crate) fn read_api_key(path: &PathBuf) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not read api_key from api_key_file {path:?}"))
        .trim()
        .to_owned()
}
