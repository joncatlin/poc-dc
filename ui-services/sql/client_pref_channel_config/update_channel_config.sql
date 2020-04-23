UPDATE dc_ui.client_pref_channel_config
SET 
    client_preference_id = $1, 
    channel_id = $2, 
    selected_permitted = $3
WHERE client_pref_channel_config_id = $4