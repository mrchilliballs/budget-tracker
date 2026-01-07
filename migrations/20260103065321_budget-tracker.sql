CREATE TABLE IF NOT EXISTS budget
(
    id          SERIAL  PRIMARY KEY,
    description TEXT    NOT NULL,
    amount      NUMERIC NOT NULL,
    category    TEXT    DEFAULT '',
    date        DATE    NOT NULL
)
