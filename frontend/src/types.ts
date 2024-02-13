export type FieldErrors<T> = {
  [K in keyof T]?: string[];
};

export type ActionState<TInput, TOutput> = {
  fieldErrors?: FieldErrors<TInput>;
  error?: {
    message?: string;
    status?: string;
  } | null;
  data?: TOutput;
};

export type Action<TInput, TOutput> = (
  data: TInput
) => Promise<ActionState<TInput, TOutput>>;

export type Article = {
  id: number;
  title: string;
  publisher_id: number;
  user_ids: number[];
  reference: string[];
  like_count: number;
  comment_count: number;
  tag_list: string[];
  published_at: Date;
  created_at: Date;
  updated_at: Date;
};

export type Comment = {
  id: number;
  author: {
    id: number;
    image: string;
    name: string;
  };
  content: string;
  likes_count: number;
  created_at: Date;
};
