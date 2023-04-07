CREATE TABLE Letter
(
    id          SERIAL PRIMARY KEY,
    subject     VARCHAR(255) NOT NULL,
    content     VARCHAR NOT NULL,
    circle_id   INTEGER REFERENCES Circle(id) NOT NULL,
    writer_id   INTEGER REFERENCES Writer(id) NOT NULL,
    reply_id    INTEGER REFERENCES Letter(id)
);
