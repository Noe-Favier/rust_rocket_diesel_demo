-- Your SQL goes here
CREATE TABLE liste (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    libelle VARCHAR(100) NOT NULL
);

INSERT INTO liste (libelle) VALUES 
    ('a'),
    ('b'),
    ('c'),
    ('d'),
    ('e');