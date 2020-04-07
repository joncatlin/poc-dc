CREATE TABLE dpl (
    dpl_id      SERIAL,
    d           VARCHAR(5),
    p           VARCHAR(5),
    l           VARCHAR(5),
    msg         TEXT NOT NULL,
    PRIMARY KEY (dpl_id),
    UNIQUE (d,p,l)
);

