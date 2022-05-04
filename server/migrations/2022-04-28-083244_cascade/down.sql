ALTER TABLE comments
  DROP CONSTRAINT comments_post_id_fkey,
  ADD CONSTRAINT comments_post_id_fkey FOREIGN KEY(post_id) REFERENCES posts(post_id)
  ON DELETE RESTRICT;

ALTER TABLE comments
  DROP CONSTRAINT comments_user_id_fkey,
  ADD CONSTRAINT comments_user_id_fkey FOREIGN KEY(user_id) REFERENCES users(user_id)
  ON DELETE RESTRICT;
