"use client";

import { z } from "zod";
import {
  CreateArticleSchema,
  SaveArticleSchema,
  UpdateArticleSchema,
} from "@/lib/definitions";
import { toast } from "sonner";
import {
  Article,
  FullArticle,
  CursorPagination,
  ResultPaging,
  ArticleVersion,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";
import { mutate } from "swr";

const delay = (delay: number) => {
  return new Promise((resolve) => setTimeout(resolve, delay));
};

export async function get_articles(option?: {
  usernames?: string[];
  list_id?: string;
  series_id?: string;
  pagination?: CursorPagination;
}): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.usernames) {
      url.append("usernames[]", option.usernames.toString());
    }
    if (option.list_id) {
      url.append("list_id", option.list_id);
    }

    if (option.series_id) {
      url.append("series_id", option.series_id);
    }

    CursorPaginationToUrlParams(url, option.pagination);
  }

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles?${url}`,
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

export async function get_article(
  article_id: string,
): Promise<FullArticle | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}`,
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

// export async function save_article(
//   article_id: string,
//   values: z.infer<typeof SaveArticleSchema>,
// ): Promise<ArticleVersion | null> {
//   return fetch(
//     `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/articles/${article_id}/edit`,
//     {
//       method: "PATCH",
//       headers: {
//         Authorization: `Bearer ${sessionStorage.getItem("session")}`,
//         "Content-Type": "application/json",
//       },
//       credentials: "include",
//       body: JSON.stringify(values),
//     },
//   ).then(async (res) => {
//     if (!res.ok) {
//       toast.error(await res.text());
//       return null;
//     }
//     return await res.json();
//   });
// }
