-- Your SQL goes here
CREATE TABLE category_mappings (
    category_mappings_id     SERIAL,
    category_id         INTEGER NOT NULL,
    correspondence_id   INTEGER NOT NULL,
    opt_out             BOOLEAN NOT NULL DEFAULT false,
    retention_period    INTEGER NOT NULL,
    PRIMARY KEY (category_id, correspondence_id),
    FOREIGN KEY (category_id) REFERENCES categories (category_id),
    FOREIGN KEY (correspondence_id) REFERENCES corrs (correspondence_id)
)
