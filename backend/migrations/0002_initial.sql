create table accounts (
  id serial primary key,
  "userId" integer not null,
  type text not null,
  provider text not null,
  "providerAccountId" text not null,
  refresh_token text,
  access_token text,
  expires_at bigint,
  token_type text,
  scope text,
  id_token text,
  session_state text
);

create table sessions (
  id serial primary key,
  "userId" integer not null,
  expires timestamptz not null,
  "sessionToken" text not null
);


create type role as enum (
  'admin',
  'user',
  'manager'
);

create table users
(
  id serial primary key ,
  name text collate "case_insensitive" not null,
  email text collate "case_insensitive" unique not null,
  "emailVerified" timestamptz,
  image text,
  approved timestamptz,
  password text,
  bio text not null default '',
  urls text[] not null default array[]::text[],
  role role not null default 'user',
  deleted boolean not null default false,
  followers_count integer not null default 0,
  following_count integer not null default 0,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

select trigger_updated_at('users');

create table follow
(
  follower_id  integer not null references "users" (id) on delete cascade,
  following_id integer not null references "users" (id) on delete cascade,
  created_at timestamptz not null default now(),
  primary key (following_id, follower_id)
);

create table interests (
  user_id integer not null ,
  tag_id integer not null,
  primary key (user_id, tag_id)
);

create type tag_status as enum (
  'approved',
  'banned',
  'waiting'
);

create table tags (
  id serial primary key,
  label text unique not null,
  tag_status tag_status not null default 'waiting',  
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table device (
  id serial primary key,
  user_id integer references "users" (id) on delete cascade,
  context text not null,
  last_logged_in_at timestamptz not null default now()
);

create table articles (
  id serial primary key,
  publisher_id integer,
  user_ids integer[] not null default array[]::integer[],
  title text,
  like_count integer not null default 0,
  comment_count integer not null default 0,
  reference text[] not null default array[]::text[],
  tag_list text[] not null default array[]::text[],
  published_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create table article_version (
  id serial primary key,
  article_id integer references articles (id) on delete cascade,
  device_id integer references device (id) on delete set null,
  version_numebr integer not null default 0,
  updated_at timestamptz not null default now()
);


create type block_type as enum (
  'text',
  'image',
  'video',
  'file'
);

create table article_block (
  id serial primary key,
  article_version_id integer references article_version (id) on delete cascade,
  block_order integer,
  block_type BLOCK_TYPE not null default 'text',
  content text not null default ''
);
