CREATE TABLE IF NOT EXISTS collections
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    src         TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS objs
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    collection  INTEGER,
    name        TEXT NOT NULL,
    src         TEXT NOT NULL UNIQUE,
    thumbnail   TEXT,
    FOREIGN KEY (collection) REFERENCES collections(id)
);
