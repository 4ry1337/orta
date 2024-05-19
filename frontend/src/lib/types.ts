export type Pagination = {
  page?: number;
  per_page?: number;
  sort?: string;
  query?: string;
};

export type Metadata = {
  first_page: number;
  last_page: number;
  per_page: number;
  page: number;
};

export type ResultPaging<T> = {
  items: T[];
  pagination: Metadata;
  total: number;
};

export type Session = {
  user_id: number;
  image?: string;
  username: string;
  role: string;
  email: string;
};

export type User = {
  id: number;
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

export type Article = {
  id: number;
  title: string;
  slug: string;
  like_count: number;
  comment_count: number;
  created_at: string;
  updated_at?: string;
  published_at?: string;
};

export type Tag = {
  id: number;
  label: string;
  slug: string;
  article_count: number;
  tag_status: string;
  created_at: string;
  updated_at?: string;
};

export type FullArticle = Article & {
  users?: User[];
  tags?: Tag[];
};

export type List = {
  id: number;
  user_id: number;
  label: string;
  slug: string;
  image?: string;
  visibility: string;
  article_count: number;
  created_at: string;
  updated_at?: string;
};

export type Series = {
  id: number;
  user_id: number;
  label: string;
  slug: string;
  image?: string;
  article_count: number;
  created_at: string;
  updated_at?: string;
};
