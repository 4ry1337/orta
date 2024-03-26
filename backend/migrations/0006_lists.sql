CREATE TABLE Lists (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES Users (id),
  slug TEXT UNIQUE NOT NULL,
  label TEXT NOT NULL,
  image TEXT,
  visibility Visibility NOT NULL DEFAULT 'PRIVATE',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Lists');

CREATE TABLE ListArticle (
  list_id INTEGER NOT NULL REFERENCES Lists (id),
  article_id INTEGER NOT NULL REFERENCES Articles (id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  PRIMARY KEY (list_id, article_id)
);
