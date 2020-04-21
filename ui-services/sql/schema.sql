DROP SCHEMA IF EXISTS dc_ui CASCADE;
CREATE SCHEMA dc_ui;

\echo 'CREATE TABLE dc_ui.channel'
CREATE TABLE dc_ui.channel (
    channel_id    SERIAL,
    channel_name  VARCHAR NOT NULL,
    PRIMARY KEY (channel_id),
    UNIQUE (channel_name)
);

\echo 'CREATE TABLE dc_ui.category'
CREATE TABLE dc_ui.category (
    category_id   SERIAL,
    category_name VARCHAR NOT NULL,
    PRIMARY KEY (category_id),
    UNIQUE (category_name)
);

\echo 'CREATE TABLE dc_ui.correspondence'
CREATE TABLE dc_ui.correspondence (
    correspondence_id   SERIAL,
    correspondence_name VARCHAR NOT NULL,
    PRIMARY KEY (correspondence_id),
    UNIQUE (correspondence_name)
);

\echo 'CREATE TABLE dc_ui.language'
CREATE TABLE dc_ui.language (
    language_id   SERIAL,
    language_name VARCHAR NOT NULL,
    PRIMARY KEY (language_id),
    UNIQUE (language_name)
);

\echo 'CREATE TABLE dc_ui.category_mapping'
CREATE TABLE dc_ui.category_mapping (
    category_mapping_id     SERIAL,
    category_id             INTEGER NOT NULL,
    correspondence_id       INTEGER NOT NULL,
    opt_out                 VARCHAR(1) NOT NULL,
    retention_period        INTEGER NOT NULL,
    PRIMARY KEY (category_mapping_id),
    FOREIGN KEY (category_id) REFERENCES dc_ui.category (category_id) ON DELETE CASCADE,
    FOREIGN KEY (correspondence_id) REFERENCES dc_ui.correspondence (correspondence_id) ON DELETE CASCADE,
    UNIQUE (category_id, correspondence_id)
);

\echo 'CREATE TABLE dc_ui.channel_config'
CREATE TABLE dc_ui.channel_config (
    channel_config_id       SERIAL,
    category_mapping_id     INTEGER NOT NULL,
    channel_id              INTEGER NOT NULL,
    permitted               VARCHAR(1) NOT NULL,
    PRIMARY KEY (channel_config_id),
    FOREIGN KEY (channel_id) REFERENCES dc_ui.channel (channel_id) ON DELETE CASCADE,
    FOREIGN KEY (category_mapping_id) REFERENCES dc_ui.category_mapping (category_mapping_id) ON DELETE CASCADE,
    UNIQUE (category_mapping_id, channel_id)
);

\echo 'CREATE TABLE dc_ui.client_preference'
CREATE TABLE dc_ui.client_preference (
    client_preference_id            SERIAL,
    category_id                     INTEGER NOT NULL,
    correspondence_id               INTEGER NOT NULL,
    selected_opt_out                VARCHAR(1) NOT NULL,
    selected_retention_period       INTEGER NOT NULL,
    developer                       VARCHAR(5) NOT NULL,
    project                         VARCHAR(5) NOT NULL,
    lender                          VARCHAR(5) NOT NULL,
    PRIMARY KEY (client_preference_id),
    FOREIGN KEY (category_id) REFERENCES dc_ui.category (category_id) ON DELETE CASCADE,
    FOREIGN KEY (correspondence_id) REFERENCES dc_ui.correspondence (correspondence_id) ON DELETE CASCADE,
    UNIQUE (category_id, correspondence_id, developer, project, lender)
);

\echo 'CREATE TABLE dc_ui.client_pref_channel_config'
CREATE TABLE dc_ui.client_pref_channel_config (
    client_pref_channel_config_id       SERIAL,
    client_preference_id                INTEGER NOT NULL,
    channel_id                          INTEGER NOT NULL,
    selected_permitted                  VARCHAR(1) NOT NULL,
    PRIMARY KEY (client_pref_channel_config_id),
    FOREIGN KEY (channel_id) REFERENCES dc_ui.channel (channel_id) ON DELETE CASCADE,
    FOREIGN KEY (client_preference_id) REFERENCES dc_ui.client_preference (client_preference_id) ON DELETE CASCADE,
    UNIQUE (client_preference_id, channel_id)
);

