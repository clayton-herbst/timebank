CREATE TABLE users (
	id TEXT PRIMARY KEY NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT,
	dob TEXT
);

CREATE TABLE activities (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    short_description TEXT,
    start_date INTEGER NOT NULL,
    end_date INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    status_id INTEGER NOT NULL
);

CREATE TABLE categories (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE statuses (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

INSERT INTO categories (name) VALUES
    ('Workout'),
    ('Reading'),
    ('Work'),
    ('Social'),
    ('Relax'),
    ('Personal Development');

INSERT INTO statuses (name) VALUES
    ('In Progress'),
    ('Complete'),
    ('Not Started'),
    ('Planning');


