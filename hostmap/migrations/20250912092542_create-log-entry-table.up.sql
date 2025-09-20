CREATE TABLE log_entry (
    log_entry_id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    host_group_name TEXT NOT NULL,
    host_name TEXT NOT NULL,
    username TEXT NOT NULL,
    store_path TEXT NOT NULL,
    activation_type TEXT NOT NULL,
    FOREIGN KEY (host_group_name, host_name)
        REFERENCES host(host_group_name, host_name)
        ON UPDATE CASCADE ON DELETE CASCADE,
	UNIQUE (host_group_name, host_name, username, timestamp, store_path, activation_type)
);
