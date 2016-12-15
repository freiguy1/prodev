CREATE TABLE worksheet (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name VARCHAR(64) NOT NULL
);

CREATE TABLE activity (
    id INTEGER PRIMARY KEY,
    worksheet_id INTEGER NOT NULL,
    description VARCHAR(128) NOT NULL,
    note TEXT NULL
);
