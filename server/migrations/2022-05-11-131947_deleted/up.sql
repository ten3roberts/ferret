ALTER TABLE posts ADD COLUMN deleted BOOLEAN;
ALTER TABLE comments ADD COLUMN deleted BOOLEAN;

UPDATE posts SET deleted = false;
UPDATE comments SET deleted = false;

ALTER TABLE posts ALTER COLUMN deleted SET NOT NULL;
ALTER TABLE comments ALTER COLUMN deleted SET NOT NULL;
