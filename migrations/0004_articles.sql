CREATE TABLE Articles (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  like_count INTEGER NOT NULL DEFAULT 0,
  comment_count INTEGER NOT NULL DEFAULT 0,
  content TEXT NOT NULL DEFAULT '',
  published_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Articles');

CREATE TABLE Authors (
  author_id TEXT references users (id) on delete cascade on update cascade,
  article_id TEXT references articles (id) on delete cascade on update cascade,
  is_owner BOOL NOT NULL DEFAULT 'FALSE',
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

CREATE TABLE Likes (
  user_id text references users (id) on delete set null on update cascade,
  article_id text references articles (id) on delete cascade on update cascade,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  PRIMARY KEY (user_id, article_id)
)
