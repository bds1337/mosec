-- Add up migration script here
CREATE TABLE IF NOT EXISTS audits (
    id CHARACTER(36) NOT NULL PRIMARY KEY,
    subject_from VARCHAR(60),
    subject_to VARCHAR(60),
    time DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    msg VARCHAR(60),
    encrypted BOOLEAN
);
