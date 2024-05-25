import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { Pagination } from "./types";
import { format } from "date-fns";
import slugify from "slugify";

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

export function isHTMLElement(x: unknown): x is HTMLElement {
  return x instanceof HTMLElement;
}

export const CAN_USE_DOM: boolean =
  typeof window !== "undefined" &&
  typeof window.document !== "undefined" &&
  typeof window.document.createElement !== "undefined";
