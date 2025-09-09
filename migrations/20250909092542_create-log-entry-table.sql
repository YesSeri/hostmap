CREATE TABLE log_entry (
	log_entry_id serial PRIMARY KEY,
	timestamp TIMESTAMPTZ NOT NULL,
	username TEXT NOT NULL,
	store_path TEXT NOT NULL,
	activation_type TEXT NOT NULL,
	UNIQUE(timestamp, username, store_path, activation_type)
);

