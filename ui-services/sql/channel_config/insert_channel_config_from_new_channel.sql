INSERT INTO dc_ui.channel_config (category_mapping_id, channel_id, permitted)
SELECT category_mapping_id, $1, '' FROM dc_ui.category_mapping;
