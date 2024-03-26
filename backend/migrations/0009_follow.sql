CREATE TABLE Follow (
  follower_id INTEGER NOT NULL REFERENCES Users (id) ON DELETE CASCADE,
  following_id INTEGER NOT NULL REFERENCES Users (id) ON DELETE CASCADE,
  created_at timestamptz not null default now (),
  updated_at timestamptz,
  constraint user_cannot_follow_self check (follower_id != following_id),
  PRIMARY KEY (follower_id, following_id)
);
