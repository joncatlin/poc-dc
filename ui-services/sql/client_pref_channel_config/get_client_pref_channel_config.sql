SELECT 
    client_pref_channel_config_id,
    client_preference_id,
    chan.channel_id AS channel_id,
    chan.channel_name AS channel_name,
    selected_permitted
FROM dc_ui.client_pref_channel_config AS cfg
RIGHT OUTER JOIN dc_ui.channel AS chan ON cfg.channel_id = chan.channel_id
WHERE cfg.client_preference_id = $1
ORDER BY chan.channel_name
