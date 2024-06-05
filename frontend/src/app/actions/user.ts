"use client";

import { UpdateUserFormSchema } from "@/lib/definitions";
import { CursorPagination, FullUser, ResultPaging, User } from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";
import { toast } from "sonner";
import { z } from "zod";

export async function get_user(username: string): Promise<User> {
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

export async function get_users(
  cursor?: CursorPagination,
): Promise<ResultPaging<FullUser>> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users`, {
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

export async function follow(username: string) {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/follow`,
      {
        method: "PUT",
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
