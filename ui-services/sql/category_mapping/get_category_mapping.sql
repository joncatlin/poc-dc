SELECT 
    cm.category_mapping_id,
    cat.category_id AS category_id,
    cat.category_name AS category_name,
    cor.correspondence_id AS correspondence_id,
    cor.correspondence_name AS correspondence_name,
    opt_out,
    retention_period,
	ccfg.channel_config_id,
	ccfg.channel_id,
    chan.channel_name,
	ccfg.permitted
FROM dc_ui.category_mapping AS cm
INNER JOIN dc_ui.category AS cat ON cm.category_id = cat.category_id
INNER JOIN dc_ui.correspondence AS cor ON cm.correspondence_id = cor.correspondence_id
INNER JOIN dc_ui.channel_config AS ccfg ON cm.category_mapping_id = ccfg.category_mapping_id
INNER JOIN dc_ui.channel AS chan ON ccfg.channel_id = chan.channel_id
WHERE cm.category_id = $1 AND cm.correspondence_id = $2;
