-- Initial Migration for Creating Database Schema

-- Player Data Tables
CREATE TABLE player (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    created_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    username           TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email              TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    email_is_verified  INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    role               INTEGER  DEFAULT 0 NOT NULL -- Enum(Guest=0, Player=1, GM=2, Admin=3)
) STRICT;

CREATE TABLE credential (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    userId             TEXT     NOT NULL UNIQUE REFERENCES user(id),
    passwordHash       TEXT     NOT NULL
) STRICT;

CREATE TABLE character (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    created_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL COLLATE NOCASE,
    home_world_id      INTEGER  NOT NULL REFERENCES world(id),
    player_id          INTEGER  NOT NULL REFERENCES player(id),
    guild_id           INTEGER  REFERENCES guild(id),
    UNIQUE(name, home_world_id)
) STRICT;

CREATE TABLE world (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    created_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    logical_server     TEXT     NOT NULL
) STRICT;

CREATE TABLE guild (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    created_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at          INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    owner_player_id    INTEGER  NOT NULL REFERENCES player(id)
) STRICT;

CREATE TABLE friendship (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    character_1_id     INTEGER  NOT NULL REFERENCES character(id),
    character_2_id     INTEGER  NOT NULL REFERENCES character(id)
) STRICT;
CREATE UNIQUE INDEX uqique_friendship_index ON friendship(character_1_id, character_2_id);

CREATE TABLE item_instance (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    character_id       INTEGER  NOT NULL REFERENCES character(id),
    item_id            INTEGER  NOT NULL REFERENCES content_item(id),
    quantity           INTEGER  NOT NULL,
    location           INTEGER  DEFAULT 0 NOT NULL, -- Enum(Equipped=0, Inventory=1, Box=2)
    in_collection      INTEGER  DEFAULT 0 NOT NULL, -- Boolean
    instance_data      TEXT -- JSON
) STRICT;

-- Game Content Tables
CREATE TABLE content_item (
    id                 INTEGER  NOT NULL PRIMARY KEY,-- Alias of rowid
    created_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    updated_at         INTEGER  DEFAULT (unixepoch()) NOT NULL, -- Unix Timestamp
    name               TEXT     NOT NULL UNIQUE COLLATE NOCASE,
    stack_size         INTEGER  DEFAULT 1 NOT NULL,
    type               INTEGER  DEFAULT 0 NOT NULL, -- Enum(Currency=0, Material=1, Consumable=2, QuestItem=3, UnlockItem=4, Equipment=5)
    icon_path          TEXT,
    drop_model_path    TEXT,
    item_data          TEXT -- JSON
) STRICT;
