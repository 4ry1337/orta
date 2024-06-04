CREATE TYPE Visibility AS ENUM ('PRIVATE', 'PUBLIC', 'BYLINK');

CREATE TABLE Lists (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES Users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  label TEXT NOT NULL,
  image TEXT,
  visibility Visibility NOT NULL DEFAULT 'PRIVATE',
  article_count INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Lists');

CREATE TABLE ListArticle (
  list_id TEXT REFERENCES Lists (id) ON UPDATE CASCADE ON DELETE CASCADE,
  article_id TEXT REFERENCES Articles (id) ON UPDATE CASCADE ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  PRIMARY KEY (list_id, article_id)
);
