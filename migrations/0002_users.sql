CREATE TYPE Role AS ENUM ('USER', 'ADMIN', 'MANAGER');

CREATE TYPE TagStatus AS ENUM ('APPROVED', 'BANNED', 'WAITING');

CREATE TYPE Visibility AS ENUM ('PRIVATE', 'PUBLIC', 'BYLINK');

CREATE TABLE Users (
  id SERIAL NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  email_verified TIMESTAMPTZ,
  image TEXT,
  role Role NOT NULL DEFAULT 'USER',
  bio TEXT,
  urls TEXT[],
  follower_count INTEGER NOT NULL DEFAULT 0,
  following_count INTEGER NOT NULL DEFAULT 0,
  approved_at TIMESTAMPTZ,
  deleted_at TIMESTAMPTZ
);

CREATE TABLE Accounts (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES Users(id) ON DELETE CASCADE ON UPDATE CASCADE,
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
  salt TEXT 
);

CREATE UNIQUE INDEX Account_provider_provider_account_id_key ON Accounts(provider, provider_account_id);
