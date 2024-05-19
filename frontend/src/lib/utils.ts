import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { Pagination } from "./types";
import { format } from "date-fns";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const PaginationToUrlParams = (
  url: URLSearchParams,
  pagination?: Pagination,
) => {
  if (pagination) {
    if (pagination.page) {
      url.append("page", pagination.page.toString());
    }
    if (pagination.per_page) {
      url.append("per_page", pagination.per_page.toString());
    }
    if (pagination.query) {
      url.append("query", pagination.query);
    }
    if (pagination.sort) {
      url.append("sort", pagination.sort);
    }
  }
};

export const now = () => (Date.now() / 1000) | 0;

export const DisplayDate = (date: string): string => {
  return format(new Date(date), "MMM d, y");
};
