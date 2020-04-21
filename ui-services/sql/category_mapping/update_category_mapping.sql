UPDATE dc_ui.category_mapping
SET 
    category_id = $1,
    correspondence_id = $2,
    opt_out = $3,
    retention_period = $4
WHERE category_mapping_id = $5
