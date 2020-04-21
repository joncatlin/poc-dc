-- Your SQL goes here
CREATE TABLE channel_configs (
    channel_config_id       SERIAL,
    category_mappings_id    INTEGER NOT NULL,
    channel_id              INTEGER NOT NULL,
    permitted               VARCHAR(1) NOT NULL,
    PRIMARY KEY (channel_config_id),
    FOREIGN KEY (channel_id) REFERENCES channels (channel_id),
    FOREIGN KEY (category_mappings_id) REFERENCES category_mappings (category_mappings_id)
)
