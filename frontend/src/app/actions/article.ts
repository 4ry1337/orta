"use client";

import { z } from "zod";
import { CreateArticleSchema, UpdateArticleSchema } from "@/lib/definitions";
import { toast } from "sonner";
import {
  Article,
  FullArticle,
  CursorPagination,
  ResultPaging,
  ArticleVersion,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_articles(option?: {
  query?: string;
  usernames?: string[];
  lists?: string[];
  serieses?: string[];
  not_lists?: string[];
  not_serieses?: string[];
  cursor?: CursorPagination;
}): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.query) {
      url.append("query", option.query);
    }

    if (option.usernames) {
      option.usernames.map((username) => {
        url.append("usernames", username);
      });
    }

    if (option.lists) {
      option.lists.map((list) => {
        url.append("lists", list);
      });
    }

    if (option.serieses) {
      option.serieses.map((series) => {
        url.append("serieses", series);
      });
    }

    if (option.not_lists) {
      option.not_lists.map((list) => {
        url.append("not_lists", list);
      });
    }

    if (option.not_serieses) {
      option.not_serieses.map((series) => {
        url.append("not_serieses", series);
      });
    }
    CursorPaginationToUrlParams(url, option.cursor);
  }

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
