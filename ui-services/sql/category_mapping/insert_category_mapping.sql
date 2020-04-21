INSERT INTO dc_ui.category_mapping(category_id, correspondence_id, opt_out, retention_period)
VALUES ($1, $2, $3, $4)
RETURNING category_mapping_id;
