-- Your SQL goes here
CREATE TABLE corrs (
  correspondence_id SERIAL,
  correspondence_name VARCHAR NOT NULL,
  PRIMARY KEY (correspondence_id),
  UNIQUE (correspondence_name)
)