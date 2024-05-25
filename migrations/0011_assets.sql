CREATE TABLE Assets (
  id TEXT DEFAULT nanoid () PRIMARY KEY,
  type TEXT NOT NULL,
  filename TEXT NOT NULL,
  permalink TEXT NOT NULL,
  created_at timestamptz not null default now (),
);
