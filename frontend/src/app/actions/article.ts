"use client";

import { z } from "zod";
import { CreateArticleSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { Article, FullArticle, Pagination, ResultPaging } from "@/lib/types";
import { PaginationToUrlParams } from "@/lib/utils";

export async function get_articles(option?: {
  usernames?: string[];
  list_id?: string;
  series_id?: string;
  pagination?: Pagination;
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

    PaginationToUrlParams(url, option.pagination);
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
