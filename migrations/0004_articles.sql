-- Add migration script here
CREATE TABLE Articles (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  title TEXT NOT NULL,
  like_count INTEGER NOT NULL DEFAULT 0,
  comment_count INTEGER NOT NULL DEFAULT 0,
  published_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Articles');

CREATE TABLE Authors (
  author_id TEXT REFERENCES Users (id) ON DELETE SET NULL ON UPDATE CASCADE,
  article_id TEXT REFERENCES Articles (id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  PRIMARY KEY (author_id, article_id)
);

CREATE TABLE Devices (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  user_id TEXT REFERENCES Users (id) ON DELETE SET NULL ON UPDATE CASCADE,
  context TEXT,
  last_logged TIMESTAMPTZ NOT NULL DEFAULT now ()
);

CREATE TABLE ArticleVersions (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  article_id TEXT NOT NULL REFERENCES Articles (id) ON DELETE CASCADE ON UPDATE CASCADE,
  device_id TEXT REFERENCES Devices (id) ON DELETE CASCADE ON UPDATE CASCADE,
  content TEXT NOT NULL DEFAULT '',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
);
