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
    category_id TEXT NOT NULL,
    status_id TEXT NOT NULL
);

CREATE TABLE categories (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE statuses (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

INSERT INTO categories (id,name) VALUES
    ('workout', 'Workout'),
    ('reading', 'Reading'),
    ('work', 'Work'),
    ('social', 'Social'),
    ('relax', 'Relax'),
    ('personal-development', 'Personal Development');

INSERT INTO statuses (id, name) VALUES
    ('in-progress', 'In Progress'),
    ('complete', 'Complete'),
    ('not-started', 'Not Started'),
    ('planning', 'Planning');


