CREATE TABLE log_entry (
	log_entry_id serial PRIMARY KEY,
	timestamp TIMESTAMPTZ NOT NULL,
	host_id BIGINT NOT NULL REFERENCES host(host_id) ON DELETE CASCADE,
	username TEXT NOT NULL,
	store_path TEXT NOT NULL,
	activation_type TEXT NOT NULL,
	UNIQUE(timestamp, username, store_path, activation_type, host_id)
);

