CREATE TABLE IF NOT EXISTS skills
(
    id INTEGER PRIMARY KEY NOT NULL,
    player_id INTEGER NOT NULL,
    skill_index SMALLINT NOT NULL,
    grade TINYINT NOT NULL
);