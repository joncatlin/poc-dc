-- Your SQL goes here
CREATE TABLE templates (
    template_id         SERIAL,
    template_name       VARCHAR (100) NOT NULL,
    language_id         INTEGER NOT NULL,
    PRIMARY KEY (template_id),
    UNIQUE (template_name),
    FOREIGN KEY (language_id) REFERENCES languages (language_id)
)
