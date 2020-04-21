SELECT 
    client_preference_id,
    cat.category_id AS category_id,
    cat.category_name AS category_name,
    cor.correspondence_id AS correspondence_id,
    cor.correspondence_name AS correspondence_name,
    opt_out,
    retention_period
FROM dc_ui.client_preference AS cm
INNER JOIN dc_ui.category AS cat ON cm.category_id = cat.category_id
INNER JOIN dc_ui.correspondence AS cor ON cm.correspondence_id = cor.correspondence_id
WHERE cm.category_id = $1
ORDER BY correspondence_name ASC;