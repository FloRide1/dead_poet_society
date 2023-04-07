CREATE TABLE WriterCircle
(
    circle_id   INTEGER REFERENCES Circle(id) NOT NULL,
    writer_id   INTEGER REFERENCES Writer(id) NOT NULL,
    PRIMARY KEY(circle_id, writer_id)
);
