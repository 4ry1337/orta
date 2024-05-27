"use client";

import { CreateSeriesSchema } from "@/lib/definitions";
import { List, CursorPagination, ResultPaging } from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";
import { toast } from "sonner";
import { z } from "zod";

export async function get_series(option?: {
  user_id?: string;
  label?: string;
  cursor?: CursorPagination;
}): Promise<ResultPaging<List>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.user_id) {
      url.append("user_id", option.user_id.toString());
    }
    if (option.label) {
      url.append("label", option.label);
    }

    CursorPaginationToUrlParams(url, option.cursor);
  }

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/series?${url}`).then(
    async (res) => {
      if (!res.ok) {
        throw new Error(`${res.status} - ${await res.text()}`);
      }
      return await res.json();
    },
  );
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
