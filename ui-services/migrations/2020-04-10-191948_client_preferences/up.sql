CREATE TABLE client_preferences (
    client_preferences_id   SERIAL,
    category_id             INTEGER NOT NULL,
    correspondence_id       INTEGER NOT NULL,
    opt_out                 VARCHAR(1) NOT NULL,
    retention_period        INTEGER NOT NULL,
    developer               VARCHAR(5) NOT NULL,
    project                 VARCHAR(5) NOT NULL,
    lender                  VARCHAR(5) NOT NULL,
    PRIMARY KEY (client_preferences_id),
    FOREIGN KEY (category_id) REFERENCES categories (category_id),
    FOREIGN KEY (correspondence_id) REFERENCES corrs (correspondence_id),
    UNIQUE (category_id, correspondence_id, developer, project, lender)
)
