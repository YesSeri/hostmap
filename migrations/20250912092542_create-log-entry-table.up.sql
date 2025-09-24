CREATE TABLE log_entry (
    log_entry_id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    hostname TEXT NOT NULL,
    username TEXT NOT NULL,
    store_path TEXT NOT NULL,
    activation_type TEXT NOT NULL,
    FOREIGN KEY (hostname)
        REFERENCES host(hostname)
        ON UPDATE CASCADE ON DELETE CASCADE,
	UNIQUE (hostname, username, timestamp, store_path, activation_type)
);
