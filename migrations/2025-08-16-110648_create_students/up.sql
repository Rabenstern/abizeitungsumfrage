-- Your SQL goes here
CREATE TABLE "student"
(
    "id"         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "email"      TEXT    NOT NULL,
    "token"      TEXT    NOT NULL,
    "first_name" TEXT    NOT NULL,
    "last_name"  TEXT    NOT NULL
);

CREATE TABLE "teacher"
(
    "id"         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "first_name" TEXT    NOT NULL,
    "last_name"  TEXT    NOT NULL
);
