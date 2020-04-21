UPDATE dc_ui.client_preference
SET 
    category_id = $1,
    correspondence_id = $2,
    selected_opt_out = $3,
    selected_retention_period = $4,
    developer = $5,
    project = $6,
    lender = $7
WHERE client_preference_id = $8
