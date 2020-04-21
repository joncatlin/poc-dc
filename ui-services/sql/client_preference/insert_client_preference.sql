INSERT INTO dc_ui.client_preference(category_id, correspondence_id, selected_opt_out, selected_retention_period, developer, project, lender)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING client_preference_id;