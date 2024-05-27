"use client";

import { z } from "zod";
import { CreateListSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { List, CursorPagination, ResultPaging } from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_lists(option?: {
  username?: string;
  cursor?: CursorPagination;
}): Promise<ResultPaging<List>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.username) {
      url.append("username", option.username);
    }

    CursorPaginationToUrlParams(url, option.cursor);
  }

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/list?${url}`).then(
    async (res) => {
      if (!res.ok) {
        throw new Error(`${res.status} - ${await res.text()}`);
      }
      return await res.json();
    },
  );
}

export async function create_list(
  values: z.infer<typeof CreateListSchema>,
): Promise<List | null> {
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/list`, {
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
