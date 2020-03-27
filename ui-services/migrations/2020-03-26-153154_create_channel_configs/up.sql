-- Your SQL goes here
CREATE TABLE channel_configs (
    channel_config_id   SERIAL,
    channel_id          INTEGER NOT NULL,
    permitted           INTEGER NOT NULL,
    language_id         INTEGER NOT NULL,
    PRIMARY KEY (channel_config_id),
    FOREIGN KEY (channel_id) REFERENCES channels (channel_id),
    FOREIGN KEY (language_id) REFERENCES languages (language_id)
)
