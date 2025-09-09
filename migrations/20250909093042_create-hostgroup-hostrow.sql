CREATE TABLE host_groups (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE hosts (
    id SERIAL PRIMARY KEY,
    host_group_id INTEGER REFERENCES host_groups(id),
    host TEXT NOT NULL,
);
