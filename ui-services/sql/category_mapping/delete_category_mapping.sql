DELETE FROM dc_ui.category_mapping
WHERE category_mapping_id = $1
RETURNING category_id;