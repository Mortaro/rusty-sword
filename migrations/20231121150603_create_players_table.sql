CREATE TABLE IF NOT EXISTS players
(
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    name VARCHAR(16)  NOT NULL,
    class TINYINT NOT NULL,
    specialty TINYINT NOT NULL,
    level TINYINT NOT NULL,
    base_strength SMALLINT NOT NULL,
    base_health SMALLINT NOT NULL,
    base_intelligence SMALLINT NOT NULL,
    base_wisdom SMALLINT NOT NULL,
    base_agility SMALLINT NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z INTEGER NOT NULL,
    face TINYINT NOT NULL,
    hair TINYINT NOT NULL,
    weapon_index SMALLINT NOT NULL,
    shield_index SMALLINT NOT NULL,
    helmet_index SMALLINT NOT NULL,
    chest_index SMALLINT NOT NULL,
    shorts_index SMALLINT NOT NULL,
    gloves_index SMALLINT NOT NULL,
    boots_index SMALLINT NOT NULL,
    current_health_points INTEGER NOT NULL,
    maximum_health_points INTEGER NOT NULL,
    current_magic_points SMALLINT NOT NULL,
    maximum_magic_points SMALLINT NOT NULL,
    experience INTEGER NOT NULL,
    rage INTEGER NOT NULL
);