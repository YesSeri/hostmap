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
    deployed_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (store_path) REFERENCES nix_store_path (store_path) ON DELETE CASCADE,
    PRIMARY KEY (
      store_path,
      commit_hash,
      branch,
      deployed_at
    )
  );
  CREATE INDEX IdxNixGitMappingStorePathBranchDeployedAt ON nix_git_link (store_path, branch, deployed_at);
  COMMIT TRANSACTION;