CREATE TABLE IF NOT EXISTS options
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    poll_id TEXT NOT NULL,
    FOREIGN KEY(poll_id) REFERENCES polls(id)
);