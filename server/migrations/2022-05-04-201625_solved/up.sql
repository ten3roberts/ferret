CREATE TABLE solved_metas (
  post_id SERIAL PRIMARY KEY REFERENCES posts,
  comment_id SERIAL REFERENCES comments
);
