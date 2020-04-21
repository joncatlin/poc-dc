UPDATE dc_ui.channel_config
SET 
    category_mapping_id = $1, 
    channel_id = $2, 
    permitted = $3
WHERE channel_config_id = $4