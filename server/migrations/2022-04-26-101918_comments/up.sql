CREATE TABLE comments (
  comment_id SERIAL PRIMARY KEY,
  post_id SERIAL NOT NULL REFERENCES posts,
  user_id VARCHAR NOT NULL REFERENCES users,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)
