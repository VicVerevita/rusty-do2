CREATE TABLE users (
	pk SERIAL PRIMARY KEY,
	user_id UUID DEFAULT gen_random_uuid() UNIQUE,
	username VARCHAR NOT NULL UNIQUE,
	email VARCHAR NOT NULL UNIQUE,
	password_hash VARCHAR NOT NULL
);

CREATE TABLE tasks (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	description VARCHAR,
	finished BOOL NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	user_id UUID NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(user_id)
);