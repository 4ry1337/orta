"use client";

import { z } from "zod";
import { CreateCommentSchema, CreateListSchema } from "@/lib/definitions";
import { toast } from "sonner";
import {
  List,
  CursorPagination,
  ResultPaging,
  FullComment,
  FullArticle,
  Comment,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_lists(
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<List>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists?${url}`, {
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

export async function get_list(list_id: string): Promise<List | null> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}`, {
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
    },
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(`${res.status} - ${await res.text()}`);
      return null;
    }
    return await res.json();
  });
}

export async function get_list_articles(
  list_id: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}/articles?${url}`,
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

export async function create_list_comment(
  list_id: string,
  values: z.infer<typeof CreateCommentSchema>,
): Promise<Comment> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}/comments`,
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

export async function get_list_comments(
  list_id: string,
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullComment>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}/comments?${url}`,
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

export async function create_list(
  values: z.infer<typeof CreateListSchema>,
): Promise<List | null> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(values),
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  });
}

export async function delete_list(list_id: string) {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}`, {
    method: "DELETE",
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
    },
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}

export async function add_article(list_id: string, article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}/articles`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        article_id: article_id,
      }),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}

export async function remove_article(list_id: string, article_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/lists/${list_id}/articles`,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        article_id: article_id,
      }),
    },
  ).then(async (res) => {
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  });
}
