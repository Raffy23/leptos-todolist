CREATE TABLE IF NOT EXISTS Users
(
    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
    username    TEXT        UNIQUE NOT NULL,
    password    TEXT        NOT NULL
);