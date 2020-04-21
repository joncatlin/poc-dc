/* Script to populate the database with static data for testing purposes */

/* Start with a fresh schema */
\i sql/schema.sql

/* CLIENT SIDE. Hence the \copy */
/* Copy data from .csv files and load them into the DB. Then reset the sequence number on the table so it does not conflict with other data in the future */
\echo 'populating data for: '
\copy dc_ui.channel FROM 'sql/data/channel.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.channel', 'channel_id'), (SELECT MAX(channel_id) FROM dc_ui.channel)+1);


\echo 'populating data for: dc_ui.correspondence'
\copy dc_ui.correspondence FROM 'sql/data/correspondence.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.correspondence', 'correspondence_id'), (SELECT MAX(correspondence_id) FROM dc_ui.correspondence)+1);


\echo 'populating data for: dc_ui.category'
\copy dc_ui.category FROM 'sql/data/category.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.category', 'category_id'), (SELECT MAX(category_id) FROM dc_ui.category)+1);


\echo 'populating data for: dc_ui.language'
\copy dc_ui.language FROM 'sql/data/language.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.language', 'language_id'), (SELECT MAX(language_id) FROM dc_ui.language)+1);


\echo 'populating data for: dc_ui.category_mapping'
\copy dc_ui.category_mapping FROM 'sql/data/category_mapping.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.category_mapping', 'category_mapping_id'), (SELECT MAX(category_mapping_id) FROM dc_ui.category_mapping)+1);


\echo 'populating data for: dc_ui.channel_config'
\copy dc_ui.channel_config FROM 'sql/data/channel_config.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.channel_config', 'channel_config_id'), (SELECT MAX(channel_config_id) FROM dc_ui.channel_config)+1);


\echo 'populating data for: dc_ui.client_preference'
\copy dc_ui.client_preference FROM 'sql/data/client_preference.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.client_preference', 'client_preference_id'), (SELECT MAX(client_preference_id) FROM dc_ui.client_preference)+1);


\echo 'populating data for: dc_ui.client_pref_channel_config'
\copy dc_ui.client_pref_channel_config FROM 'sql/data/client_pref_channel_config.csv' WITH (DELIMITER ',', FORMAT CSV, HEADER true)
SELECT pg_catalog.setval(pg_get_serial_sequence('dc_ui.client_pref_channel_config', 'client_pref_channel_config_id'), (SELECT MAX(client_pref_channel_config_id) FROM dc_ui.client_pref_channel_config)+1);
