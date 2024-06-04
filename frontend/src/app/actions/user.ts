"use client";

import { CursorPagination, ResultPaging, User } from "@/lib/types";

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
): Promise<ResultPaging<User>> {
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
