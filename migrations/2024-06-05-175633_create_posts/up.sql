-- Your SQL goes here
CREATE TABLE peers
(
    id        uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)