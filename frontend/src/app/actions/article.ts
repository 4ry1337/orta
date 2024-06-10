"use client";

import { z } from "zod";
import {
  CreateArticleSchema,
  CreateCommentSchema,
  UpdateArticleSchema,
} from "@/lib/definitions";
import { toast } from "sonner";
import {
  Article,
  FullArticle,
  CursorPagination,
  ResultPaging,
  ArticleVersion,
  FullComment,
  Comment,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_articles(
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles?${url}`, {
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
    },
  }).then(async (res) => {
    if (!res.ok) {
      throw new Error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}

export async function get_article(
  article_id: string,
): Promise<FullArticle | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}`,
    {
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  });
}

export async function get_article_comments(
  article_id: string,
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullComment>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/comments?${url}`,
    {
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      throw new Error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}

export async function create_article_comment(
  article_id: string,
  values: z.infer<typeof CreateCommentSchema>,
): Promise<Comment> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/comments`,
    {
      method: "POST",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify(values),
    },
  ).then(async (res) => {
    if (!res.ok) {
      throw new Error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}

export async function create_article(
  values: z.infer<typeof CreateArticleSchema>,
): Promise<Article | null> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify(values),
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  });
}

export async function update_article(
  article_id: string,
  values: z.infer<typeof UpdateArticleSchema>,
): Promise<Article | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}`,
    {
      method: "PATCH",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify(values),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  });
}

export async function delete_article(article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}`,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}

export async function get_history(
  article_id: string,
): Promise<ArticleVersion | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/history`,
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  });
}

export async function like_article(article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/like`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}

export async function unlike_article(article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/like`,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}

export async function publish(article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/publish`,
    {
      method: "PATCH",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
    }
    toast(await res.text());
  });
}

export async function add_author(
  article_id: string,
  user_id: string,
): Promise<string | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/authors`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_id,
      }),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    const text = await res.text();
    toast(text);
    return text;
  });
}

export async function remove_author(
  article_id: string,
  user_id: string,
): Promise<string | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/authors`,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        user_id,
      }),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    const text = await res.text();
    toast(text);
    return text;
  });
}

export async function set_tags(
  article_id: string,
  add_tags: string[],
  remove_tags: string[],
): Promise<string | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/edit/tags`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        add_tags: add_tags,
        remove_tags: remove_tags,
      }),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    const text = await res.text();
    toast(text);
    return text;
  });
}
