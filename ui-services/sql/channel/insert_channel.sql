INSERT INTO dc_ui.channel(channel_name)
VALUES ($1)
RETURNING channel_id;
