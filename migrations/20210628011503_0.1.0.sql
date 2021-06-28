CREATE TABLE IF NOT EXISTS users
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    pass TEXT NOT NULL,
    salt TEXT NOT NULL,
    role TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS polls
(
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    require_name BOOLEAN NOT NULL DEFAULT 1,
    allow_participant_options BOOLEAN NOT NULL DEFAULT 0,
    poll_type TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0,
    user_id INTEGER NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS options
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    poll_id TEXT NOT NULL,

    FOREIGN KEY(poll_id) REFERENCES polls(id)
);

CREATE TABLE IF NOT EXISTS submissions
(
    id INTEGER PRIMARY KEY NOT NULL,
    participant_name TEXT,
    score INTEGER NOT NULL,
    option_id INTEGER NOT NULL,
    poll_id TEXT NOT NULL,

    FOREIGN KEY(poll_id) REFERENCES polls(id),
    FOREIGN KEY(option_id) REFERENCES options(id)
);

CREATE TABLE IF NOT EXISTS sessions
(
    id TEXT PRIMARY KEY NOT NULL,
    expires INTEGER NULL,
    data TEXT NOT NULL
);
