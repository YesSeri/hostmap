BEGIN TRANSACTION;
CREATE TABLE
  NixStorePath (
    store_path TEXT PRIMARY KEY,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
  );

CREATE TABLE
  NixGitMapping (
    hostname TEXT NOT NULL,
    store_path TEXT NOT NULL,
    commit_hash TEXT NOT NULL,
    branch TEXT NOT NULL,
    deployed_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (host_name) REFERENCES host (hostname) ON DELETE CASCADE,
    FOREIGN KEY (store_path) REFERENCES NixStorePath (store_path) ON DELETE CASCADE,
    PRIMARY KEY (
      host_name,
      store_path,
      commit_hash,
      branch,
      deployed_at
    )
  );
  CREATE INDEX IdxNixGitMappingHostNameDeployedAt ON NixGitMapping (hostname, deployed_at);
  COMMIT TRANSACTION;