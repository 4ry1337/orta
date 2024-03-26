-- Add migration script here
CREATE TABLE Articles (
  id SERIAL NOT NULL PRIMARY KEY,
  slug TEXT UNIQUE NOT NULL,
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
  author_id INTEGER REFERENCES Users (id) ON DELETE SET NULL ON UPDATE CASCADE,
  article_id INTEGER REFERENCES Articles (id) ON DELETE CASCADE ON UPDATE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  PRIMARY KEY (author_id, article_id)
);

CREATE TABLE Devices (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER REFERENCES Users (id) ON DELETE SET NULL ON UPDATE CASCADE,
  context TEXT,
  last_logged TIMESTAMPTZ NOT NULL DEFAULT now ()
);

CREATE TABLE ArticleVersions (
  id SERIAL NOT NULL PRIMARY KEY,
  article_id INTEGER NOT NULL REFERENCES Articles (id) ON DELETE CASCADE ON UPDATE CASCADE,
  device_id INTEGER NOT NULL REFERENCES Devices (id),
  version_number INTEGER NOT NULL DEFAULT 0,
  content TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
);
