SELECT
    cp.client_preference_id,
    cp.developer,
    cp.project,
    cp.lender,
    cm.category_id,
    c.category_name,
    cm.correspondence_id,
    cor.correspondence_name,
    cm.opt_out,
    cp.selected_opt_out,
    cm.retention_period,
    cp.selected_retention_period,
    cc.channel_config_id,
    cc.channel_id,
    ch.channel_name,
    cc.permitted,
    cpcc.selected_permitted,
    cpcc.client_pref_channel_config_id
FROM dc_ui.client_preference cp
INNER JOIN dc_ui.category_mapping AS cm
    ON cm.category_id  = cp.category_id
    AND cm.correspondence_id = cp.correspondence_id
INNER JOIN dc_ui.channel_config cc
    ON cc.category_mapping_id = cm.category_mapping_id
INNER JOIN dc_ui.category c
    ON cp.category_id = c.category_id
INNER JOIN dc_ui.correspondence cor
    ON cp.correspondence_id = cor.correspondence_id
INNER JOIN dc_ui.channel ch
    ON cc.channel_id = ch.channel_id
INNER JOIN dc_ui.client_pref_channel_config cpcc
    ON cp.client_preference_id = cpcc.client_preference_id
    AND cpcc.channel_id = ch.channel_id
$where_clause
ORDER BY cp.developer, cp.project, cp.lender, cor.correspondence_name, ch.channel_name ASC;