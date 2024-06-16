-- Your SQL goes here
CREATE TABLE users
(
  id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  username VARCHAR(255) NOT NULL UNIQUE,
  hashed_password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');
CREATE UNIQUE INDEX users_username ON users (username);