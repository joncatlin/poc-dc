SELECT 
    channel_config_id,
    category_mapping_id,
    chan.channel_id AS channel_id,
    chan.channel_name AS channel_name,
    permitted
FROM dc_ui.channel_config AS cfg
RIGHT OUTER JOIN dc_ui.channel AS chan ON cfg.channel_id = chan.channel_id
WHERE cfg.category_mapping_id = $1
ORDER BY chan.channel_name
