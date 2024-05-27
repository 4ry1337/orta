import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { CursorPagination } from "./types";
import { format } from "date-fns";
import slugify from "slugify";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const CursorPaginationToUrlParams = (
  url: URLSearchParams,
  cursor?: CursorPagination,
) => {
  if (cursor) {
    if (cursor.limit) {
      url.append("limit", cursor.limit.toString());
    }
    if (cursor.cursor) {
      url.append("cursor", cursor.cursor);
    }
  }
};

export const now = () => (Date.now() / 1000) | 0;

export const DisplayDate = (date: string): string => {
  return format(new Date(date), "MMM d, y");
};

export const get_id = (url: string): string => {
  return url.substring(url.lastIndexOf("-") + 1);
};

export const slugifier = (value: string): string => {
  return slugify(value, {
    replacement: "-",
    trim: true,
    lower: true,
  });
};
