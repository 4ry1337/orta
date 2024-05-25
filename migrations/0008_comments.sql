CREATE TYPE CommentableType AS ENUM ('ARTICLE', 'LIST', 'SERIES');

CREATE TABLE Comments (
  parent ltree,
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  commenter_id TEXT NOT NULL REFERENCES Users (id) ON DELETE SET NULL,
  target_id TEXT NOT NULL,
  type CommentableType NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Comments');

CREATE INDEX ON Comments (target_id, type);
