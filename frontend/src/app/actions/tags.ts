import { CursorPagination, ResultPaging, Tag } from "@/lib/types";
import { CursorPaginationToUrlParams } from "@/lib/utils";

export async function get_tags(option?: {
  query?: string;
  cursor?: CursorPagination;
}): Promise<ResultPaging<Tag>> {
  const url = new URLSearchParams();

  if (option) {
    if (option.query) {
      url.append("query", option.query);
    }

    CursorPaginationToUrlParams(url, option.cursor);
  }

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/tags?${url}`, {
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
