-- Add migration script here
CREATE TABLE Commands (
    time TIMESTAMP PRIMARY KEY,
    command TEXT NOT NULL, 
    signature BYTEA NOT NULL
);
