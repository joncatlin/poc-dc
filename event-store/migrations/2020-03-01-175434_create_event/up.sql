CREATE TABLE event (
   message_id           VARCHAR(20) NOT NULL,
   channel              VARCHAR(15) NOT NULL,
   event_status         VARCHAR(10) NOT NULL,
   event_timestamp      TIMESTAMP NOT NULL,
   event_specific_data  TEXT,
   CONSTRAINT pk_event PRIMARY KEY (message_id, channel)
);

