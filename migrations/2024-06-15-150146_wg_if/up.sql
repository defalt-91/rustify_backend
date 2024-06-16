-- Your SQL goes here
CREATE TABLE wg_if
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    pubkey VARCHAR(255) NOT NULL,
    privkey VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    port INTEGER NOT NULL,
    mtu INTEGER,
    fwmark INTEGER
)
