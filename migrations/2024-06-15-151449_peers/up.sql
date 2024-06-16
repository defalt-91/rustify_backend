-- Your SQL goes here
ALTER TABLE peers ADD COLUMN interface_id INTEGER NOT NULL REFERENCES wg_if(id)