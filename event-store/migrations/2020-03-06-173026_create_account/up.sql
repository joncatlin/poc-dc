CREATE TABLE account (
   message_id       VARCHAR(100) NOT NULL,
   channel          VARCHAR(15) NOT NULL,
   account_id       VARCHAR(30) NOT NULL,
   CONSTRAINT pk_account PRIMARY KEY (message_id, account_id, channel)
)