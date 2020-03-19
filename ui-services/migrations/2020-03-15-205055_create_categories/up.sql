-- Your SQL goes here
CREATE TABLE categories (
  category_id   SERIAL,
  category_name VARCHAR NOT NULL,
  PRIMARY KEY (category_id),
  UNIQUE (category_name)
)
