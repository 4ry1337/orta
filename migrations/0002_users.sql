CREATE TYPE Role AS ENUM ('USER', 'ADMIN', 'MANAGER');

CREATE TABLE Users (
  id TEXT DEFAULT nanoid() PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  email_verified TIMESTAMPTZ,
  image TEXT,
  role Role NOT NULL DEFAULT 'USER',
  bio TEXT NOT NULL DEFAULT '',
  urls TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
  follower_count INTEGER NOT NULL DEFAULT 0,
  following_count INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
  approved_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ
);

CREATE TABLE Accounts (
  id TEXT DEFAULT nanoid() PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
  type TEXT NOT NULL,
  provider TEXT NOT NULL,
  provider_account_id TEXT NOT NULL,
  refresh_token TEXT,
  access_token TEXT,
  expires_at BIGINT,
  token_type TEXT,
  scope TEXT,
  id_token TEXT,
  session_state TEXT,
  password TEXT,
  salt TEXT, 
  created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
);

CREATE TABLE Verification_token (
  token TEXT DEFAULT nanoid(15) PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
  expires_at TIMESTAMPTZ NOT NULL
);

CREATE UNIQUE INDEX Account_provider_provider_account_id_key ON Accounts(provider, provider_account_id);
