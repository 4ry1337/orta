CREATE TABLE Follow (
  follower_id TEXT NOT NULL REFERENCES Users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  following_id TEXT NOT NULL REFERENCES Users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  created_at timestamptz not null default now (),
  updated_at timestamptz,
  constraint user_cannot_follow_self check (follower_id != following_id),
  PRIMARY KEY (follower_id, following_id)
);
