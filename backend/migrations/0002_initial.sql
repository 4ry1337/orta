CREATE TYPE Role AS ENUM ('USER', 'ADMIN', 'MANAGER');

CREATE TYPE TagStatus AS ENUM ('APPROVED', 'BANNED', 'WAITING');

CREATE TABLE Users (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT,
    email TEXT UNIQUE NOT NULL,
    email_verified TIMESTAMPTZ,
    password TEXT,
    image TEXT,
    role Role NOT NULL DEFAULT 'USER',
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
    expires_at INTEGER,
    token_type TEXT,
    scope TEXT,
    id_token TEXT,
    session_state TEXT
);

CREATE UNIQUE INDEX Account_provider_provider_account_id_key ON Accounts(provider, provider_account_id);

CREATE TABLE Profiles (
    id SERIAL NOT NULL PRIMARY KEY,
    bio TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES Users(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    urls TEXT[],
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('Profiles');

CREATE TABLE Tags (
    id SERIAL NOT NULL PRIMARY KEY,
    label TEXT NOT NULL UNIQUE,
    article_count INTEGER NOT NULL DEFAULT 0,
    tag_status TagStatus NOT NULL DEFAULT 'WAITING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('Tags');

CREATE TABLE Interests (
  profile_id INTEGER NOT NULL REFERENCES Profiles(id),
  tag_id INTEGER NOT NULL REFERENCES Tags(id),
  PRIMARY KEY (profile_id, tag_id)
);

CREATE TABLE Devices (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id INTEGER REFERENCES Users(id) ON DELETE SET NULL ON UPDATE CASCADE,
    context TEXT,
    last_logged TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE Articles (
    id SERIAL NOT NULL PRIMARY KEY,
    title TEXT,
    like_count INTEGER NOT NULL DEFAULT 0,
    comment_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('Articles');

CREATE TABLE Authors (
  author_id INTEGER REFERENCES Users(id) ON DELETE SET NULL ON UPDATE CASCADE,
  article_id INTEGER REFERENCES Articles(id) ON DELETE CASCADE ON UPDATE CASCADE,
  PRIMARY KEY (author_id, article_id)
);

CREATE TABLE ArticleTags (
  article_id INTEGER REFERENCES Articles(id),
  tag_id INTEGER REFERENCES Tags(id),
  PRIMARY KEY (article_id, tag_id)
);

CREATE TABLE ArticleVersions (
    id SERIAL NOT NULL PRIMARY KEY,
    article_id INTEGER NOT NULL REFERENCES Articles(id) ON DELETE CASCADE ON UPDATE CASCADE,
    device_id INTEGER NOT NULL REFERENCES Devices(id),
    version_number INTEGER NOT NULL DEFAULT 0,
    content TEXT,
    updated_at TIMESTAMPTZ
);

SELECT trigger_updated_at('ArticleVersions');

CREATE TABLE Comments (
    id SERIAL NOT NULL PRIMARY KEY,
    content TEXT,
    article_id INTEGER REFERENCES Articles(id) ON DELETE CASCADE ON UPDATE CASCADE
);
