-- Your SQL goes here
CREATE TABLE words (
  id SERIAL PRIMARY KEY,
  word VARCHAR(128) NOT NULL,
  definition TEXT NOT NULL
);
CREATE INDEX word_index ON words (word);