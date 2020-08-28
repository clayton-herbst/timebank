CREATE TABLE users (
	id TEXT PRIMARY KEY UNIQUE,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT,
	dob TEXT
) WITHOUT ROWID;

INSERT INTO users (id, first_name, last_name, email, dob) VALUES ("Y2xheXRvbiBoZXJic3QgMDYvMDYvMTk5OQo=", "Clayton", "Herbst", "herbsca1@gmail.com", "06/06/1999");
