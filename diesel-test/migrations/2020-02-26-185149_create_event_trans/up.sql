CREATE TABLE event_trans (
  id SERIAL PRIMARY KEY,
  event_id VARCHAR(20) NOT NULL,
  event_type VARCHAR(20) NOT NULL,
  event_status VARCHAR(20) NOT NULL,
  event_datetime TIMESTAMP NOT NULL,
  event_specific_data TEXT
)