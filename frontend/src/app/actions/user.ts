"use client";

import { User } from "@/lib/types";

export async function get_user(username: string): Promise<User> {
  return fetch(
    `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}`,
  ).then(async (res) => {
    if (!res.ok) {
      throw new Error(`${res.status} - ${await res.text()}`);
    }
    return await res.json();
  });
}
