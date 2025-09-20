CREATE TABLE host_group (
  host_group_name TEXT PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE host (
  host_group_name TEXT NOT NULL REFERENCES host_group(host_group_name)
    ON UPDATE CASCADE ON DELETE CASCADE,
  host_name TEXT NOT NULL,
  host_url TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (host_group_name, host_name)
);
-- CREATE INDEX host_name_only ON host(host_name); will we look up hosts by name only?