"use client";

import { UpdateUserFormSchema } from "@/lib/definitions";
import {
  CursorPagination,
  FullArticle,
  FullUser,
  List,
  ResultPaging,
  Series,
  User,
} from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";
import { toast } from "sonner";
import { z } from "zod";

export async function get_user(username: string): Promise<FullUser> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}`, {
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

export async function get_feed(
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/me/feed?${url}`, {
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

export async function get_user_articles(
  username: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/articles?${url}`,
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

export async function get_user_lists(
  username: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<List>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/lists?${url}`,
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

export async function get_user_serieses(
  username: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<Series>> {
  const url = new URLSearchParams();

  CursorPaginationToUrlParams(url, cursor);

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/series?${url}`,
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

export async function get_user_drafts(
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullArticle>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/me/drafts?${url}`, {
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

export async function get_users(
  query?: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullUser>> {
  const url = new URLSearchParams();

  if (query) {
    url.append("query", query);
  }

  CursorPaginationToUrlParams(url, cursor);

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users?${url}`, {
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
    },
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}

export async function follow(username: string) {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/follow`,
      {
        method: "PUT",
        headers: {
          Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        },
      },
    );
    if (!res.ok) {
      toast.error(await res.text());
    }
    toast(await res.text());
  } catch (error) {
    console.error(error);
  }
}

export async function unfollow(username: string) {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/follow`,
      {
        method: "delete",
        headers: {
          Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        },
      },
    );
    if (!res.ok) {
      toast.error(await res.text());
    }
    toast(await res.text());
  } catch (error) {
    console.error(error);
  }
}

export async function get_followers(
  username: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullUser>> {
  const url = new URLSearchParams();

  if (cursor) {
    CursorPaginationToUrlParams(url, cursor);
  }

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/followers?${url}`,
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

export async function get_following(
  username: string,
  cursor?: CursorPagination,
): Promise<ResultPaging<FullUser>> {
  const url = new URLSearchParams();

  if (cursor) {
    CursorPaginationToUrlParams(url, cursor);
  }

  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/following?${url}`,
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

export async function update_user(
  username: string,
  values: z.infer<typeof UpdateUserFormSchema>,
): Promise<User | null> {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}`,
      {
        method: "PATCH",
        headers: {
          Authorization: `Bearer ${sessionStorage.getItem("session")}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify(values),
      },
    );
    if (!res.ok) {
      toast.error(`${res.status} - ${await res.text()}`);
      return null;
    }
    return await res.json();
  } catch (error) {
    return null;
  }
}
