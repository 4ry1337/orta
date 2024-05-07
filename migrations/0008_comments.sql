CREATE TYPE CommentableType AS ENUM ('ARTICLE', 'LIST', 'SERIES');

CREATE TABLE Comments (
  id SERIAL NOT NULL PRIMARY KEY,
  commenter_id INTEGER NOT NULL REFERENCES Users (id) ON DELETE SET NULL,
  target_id INTEGER NOT NULL,
  type CommentableType NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Comments');

CREATE INDEX ON Comments (target_id, type);
