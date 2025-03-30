CREATE TABLE IF NOT EXISTS Users
(
    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
    username    TEXT        UNIQUE NOT NULL,
    password    TEXT        NOT NULL
);

CREATE TABLE IF NOT EXISTS NOTES
(
    id          BLOB        PRIMARY KEY,
    owner       INTEGER     NOT NULL REFERENCES Users(id),
    title       TEXT        NOT NULL,
    content     TEXT        NOT NULL
);
