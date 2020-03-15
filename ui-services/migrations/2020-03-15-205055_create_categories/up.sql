-- Your SQL goes here
CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  category_name VARCHAR NOT NULL,
  CONSTRAINT uk_category_name UNIQUE (category_name)
)
