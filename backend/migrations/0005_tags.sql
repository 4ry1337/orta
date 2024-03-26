CREATE TABLE Tags (
  id SERIAL NOT NULL PRIMARY KEY,
  label TEXT NOT NULL UNIQUE,
  article_count INTEGER NOT NULL DEFAULT 0,
  tag_status TagStatus NOT NULL DEFAULT 'WAITING',
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Tags');

CREATE TABLE Interests (
  profile_id INTEGER NOT NULL REFERENCES Profiles (id),
  tag_id INTEGER NOT NULL REFERENCES Tags (id),
  PRIMARY KEY (profile_id, tag_id)
);

CREATE TABLE ArticleTags (
  article_id INTEGER REFERENCES Articles (id),
  tag_id INTEGER REFERENCES Tags (id),
  PRIMARY KEY (article_id, tag_id)
);
