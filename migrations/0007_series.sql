CREATE TABLE Series (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES Users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  label TEXT NOT NULL,
  image TEXT,
  article_count INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Series');

CREATE TABLE SeriesArticle (
  series_id TEXT REFERENCES Series (id) ON UPDATE CASCADE ON DELETE CASCADE,
  article_id TEXT UNIQUE REFERENCES Articles (id) ON UPDATE CASCADE ON DELETE CASCADE,
  "order" REAL NOT NULL DEFAULT 0.0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ,
  PRIMARY KEY (series_id, article_id)
);

SELECT
  trigger_updated_at ('SeriesArticle');
