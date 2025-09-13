CREATE TABLE host_group (
  host_group_id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

create table host (
  host_id BIGSERIAL PRIMARY KEY,
  host_group_id BIGINT NOT NULL REFERENCES host_group(host_group_id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (name)
);


