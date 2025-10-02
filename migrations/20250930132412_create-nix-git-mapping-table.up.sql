CREATE TABLE NixGitMapping (
    host_name TEXT NOT NULL,
    store_path TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    branch TEXT NOT NULL,
    deployed_at TIMESTAMPTZ NOT NULL,
    UNIQUE (host_name, store_path, commit_hash)
);
