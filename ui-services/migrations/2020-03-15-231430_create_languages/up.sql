-- Your SQL goes here
CREATE TABLE languages (
  id SERIAL PRIMARY KEY,
  language_name VARCHAR NOT NULL,
  CONSTRAINT uk_language_name UNIQUE (language_name)
)
