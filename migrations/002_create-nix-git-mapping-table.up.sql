BEGIN TRANSACTION;
CREATE TABLE
  nix_store_path (
    store_path TEXT PRIMARY KEY,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
  );

CREATE TABLE
  nix_git_link (
    store_path TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    branch TEXT NOT NULL,
    linked_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (store_path) REFERENCES nix_store_path (store_path) ON DELETE CASCADE,
    PRIMARY KEY (
      store_path,
      commit_hash,
      branch,
      linked_at
    )
  );
  CREATE INDEX IdxNixGitMappingStorePathBranchLinkedAt ON nix_git_link (store_path, branch, linked_at);
  COMMIT TRANSACTION;