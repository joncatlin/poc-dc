-- Your SQL goes here
CREATE TABLE channels (
  channel_id    SERIAL,
  channel_name  VARCHAR NOT NULL,
  PRIMARY KEY (channel_id),
  UNIQUE (channel_name)
)


