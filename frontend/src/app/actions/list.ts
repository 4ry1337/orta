"use client";

import { z } from "zod";
import { CreateListSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { List, CursorPagination, ResultPaging } from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_lists(option?: {
  query?: string;
  user_id?: string;
  cursor?: CursorPagination;
}): Promise<ResultPaging<List>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.query) {
      url.append("query", option.query);
    }

    if (option.user_id) {
      url.append("user_id", option.user_id);
    }

    CursorPaginationToUrlParams(url, option.cursor);
  }

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
