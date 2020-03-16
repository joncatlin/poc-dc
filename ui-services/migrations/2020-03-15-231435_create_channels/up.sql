-- Your SQL goes here
CREATE TABLE channels (
  id SERIAL PRIMARY KEY,
  channel_name VARCHAR NOT NULL,
  CONSTRAINT uk_channel_name UNIQUE (channel_name)
)
