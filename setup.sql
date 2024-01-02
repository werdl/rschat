CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    from_uname TEXT NOT NULL,
    to_uname TEXT NOT NULL, 
    msg TEXT NOT NULL, 
    ts TEXT NOT NULL
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    uname TEXT NOT NULL,
    pass_hash TEXT NOT NULL
);
