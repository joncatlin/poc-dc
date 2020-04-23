INSERT INTO dc_ui.client_pref_channel_config (client_preference_id, channel_id, selected_permitted)
SELECT client_preference_id, $1, '' FROM dc_ui.client_preference;
