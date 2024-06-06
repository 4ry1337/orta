export type IRoute = {
  label: string;
  icon: any;
  href: string;
};

export type CursorPagination = {
  limit: number;
  cursor?: string;
};

export type Metadata = {
  first_page: number;
  last_page: number;
  per_page: number;
  page: number;
};

export type ResultPaging<T> = {
  items: T[];
  next_cursor: string;
};

export type Session = {
  user_id: string;
  image?: string;
  username: string;
  role: string;
  email: string;
};

export type User = {
  id: string;
  username: string;
  email: string;
  email_verified?: string;
  image?: string;
  role: string;
  bio: string;
  urls: string[];
  follower_count: number;
  following_count: number;
  approved_at: string;
  deleted_at: string;
};

export type FullUser = {
  id: string;
  username: string;
  email: string;
  email_verified?: string;
  image?: string;
  bio: string;
  urls: string[];
  follower_count: number;
  following_count: number;
  approved_at: string;
  deleted_at: string;
  followed: boolean;
};

export type Article = {
  id: string;
  title: string;
  content?: string;
  description?: string;
  like_count: number;
  comment_count: number;
  created_at: string;
  updated_at?: string;
  published_at?: string;
};

export type ArticleVersion = {
  id: string;
  article_id: string;
  device_id?: string;
  content: string;
  created_at: string;
};

export type Tag = {
  slug: string;
  label: string;
  article_count: number;
  tag_status: string;
  created_at: string;
  updated_at?: string;
};

export type FullArticle = Article & {
  users: FullUser[];
  tags: Tag[];
  lists: List[];
  series: Series[];
  liked: boolean;
};

export type List = {
  id: string;
  user_id: string;
  label: string;
  slug: string;
  image?: string;
  visibility: string;
  article_count: number;
  created_at: string;
  updated_at?: string;
};

export type Series = {
  id: string;
  user_id: string;
  label: string;
  image?: string;
  article_count: number;
  created_at: string;
  updated_at?: string;
};

export type Comment = {
  id: string;
  target_id: string;
  user: FullUser;
};
