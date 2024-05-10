"use server";

import { SignUpFormSchema } from "@/app/lib/definitions";
import { cookies } from "next/headers";
import { z } from "zod";

export async function signup(values: z.infer<typeof SignUpFormSchema>) {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/auth/credential/signup`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(values),
      },
    );
    sessionStorage.setItem("session", await res.text());
  } catch (error) {
    return error;
  }
}

export async function signin(values: z.infer<typeof SignUpFormSchema>) {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/auth/credential/signin`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(values),
      },
    );
    sessionStorage.setItem("session", await res.text());
  } catch (error) {
    return error;
  }
}

export async function refresh() {
  try {
    const res = await fetch(`${process.env.BACKEND_URL}/api/auth/refresh`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
    });
    sessionStorage.setItem("session", await res.text());
  } catch (error) {
    return error;
  }
}

export async function signout() {
  cookies().delete("fingerprint").delete("refresh_token");
  sessionStorage.removeItem("session");
}
