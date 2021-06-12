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