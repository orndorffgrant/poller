CREATE TABLE IF NOT EXISTS polls
(
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    require_name BOOLEAN NOT NULL DEFAULT 1,
    allow_participant_options BOOLEAN NOT NULL DEFAULT 0,
    poll_type TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0
);