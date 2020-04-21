SELECT  $table_fields FROM dc_ui.correspondence WHERE NOT EXISTS (
    SELECT correspondence_id FROM dc_ui.category_mapping WHERE correspondence_id = correspondence.correspondence_id
) ORDER BY correspondence_name;
