-- Your SQL goes here
CREATE TABLE users
(
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR NOT NULL UNIQUE,
  hashed_password VARCHAR NOT NULL
)