CREATE TABLE users (
	id TEXT PRIMARY KEY UNIQUE,
	first_name TEXT,
	family_name TEXT,
	email TEXT,
	dob TEXT
) WITHOUT ROWID;

INSERT INTO users (id, first_name, family_name, email, dob) VALUES ("Y2xheXRvbiBoZXJic3QgMDYvMDYvMTk5OQo=", "Clayton", "Herbst", "herbsca1@gmail.com", "06/06/1999");