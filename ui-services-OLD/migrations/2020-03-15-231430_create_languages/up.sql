-- Your SQL goes here
CREATE TABLE languages (
  language_id SERIAL,
  language_name VARCHAR NOT NULL,
  PRIMARY KEY (language_id),
  UNIQUE (language_name)
)

