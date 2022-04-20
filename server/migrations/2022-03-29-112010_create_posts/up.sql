CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE users (
  user_id VARCHAR PRIMARY KEY,
  username VARCHAR NOT NULL
);
