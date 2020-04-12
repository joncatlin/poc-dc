-- Your SQL goes here
CREATE TABLE client_pref_channel_configs (
    client_pref_channel_config_id       SERIAL,
    client_preferences_id               INTEGER NOT NULL,
    channel_id                          INTEGER NOT NULL,
    permitted                           VARCHAR(1) NOT NULL,
    PRIMARY KEY (client_pref_channel_config_id),
    FOREIGN KEY (channel_id) REFERENCES channels (channel_id),
    FOREIGN KEY (client_preferences_id) REFERENCES client_preferences (client_preferences_id)
)
