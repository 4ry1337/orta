CREATE TABLE Comments (
  id SERIAL NOT NULL PRIMARY KEY,
  content TEXT NOT NULL,
  commenter_id INTEGER NOT NULL REFERENCES Users (id) ON DELETE SET NULL,
  article_id INTEGER REFERENCES Articles (id) ON DELETE CASCADE,
  series_id INTEGER REFERENCES Series (id) ON DELETE CASCADE,
  list_id INTEGER REFERENCES Lists (id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  updated_at TIMESTAMPTZ
);

SELECT
  trigger_updated_at ('Comments');

CREATE INDEX ON Comments (article_id, created_at);
