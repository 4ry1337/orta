"use client";

import {
  CreateCommentSchema,
  CreateSeriesSchema,
  UpdateSeriesSchema,
} from "@/lib/definitions";
import {
  List,
  CursorPagination,
  ResultPaging,
  Series,
  Comment,
  FullComment,
  FullArticle,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";
import { toast } from "sonner";
import { z } from "zod";

export async function get_serieses(
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<Series>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series?${url}`).then(
    async (res) => {
      if (!res.ok) {
        throw new Error(`${res.status} - ${await res.text()}`);
      }
      return await res.json();
    },
  );
}

export async function get_series(series_id: string): Promise<Series> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}`,
  ).then(async (res) => {
    if (!res.ok) {
      throw new Error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}

export async function get_series_articles(
  series_id: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/articles?${url}`,
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

export async function create_series_comment(
  series_id: string,
  values: z.infer<typeof CreateCommentSchema>,
): Promise<Comment> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/comments`,
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

export async function get_series_comments(
  series_id: string,
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullComment>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/comments?${url}`,
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

export async function create_series(
  values: z.infer<typeof CreateSeriesSchema>,
): Promise<List | null> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series`, {
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

export async function update_series(
  series_id: string,
  values: z.infer<typeof UpdateSeriesSchema>,
): Promise<List | null> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}`,
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

export async function delete_series(series_id: string) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}`,
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

export async function reorder_article_series(
  series_id: string,
  article_id: string,
  order: number,
) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/articles`,
    {
      method: "PATCH",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        article_id,
        order,
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

export async function add_article_series(
  series_id: string,
  article_id: string,
) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/articles`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        article_id,
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

export async function remove_article_series(
  series_id: string,
  article_id: string,
) {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series/${series_id}/articles`,
    {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        article_id,
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
