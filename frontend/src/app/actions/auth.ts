"use client";

import { SignInFormSchema, SignUpFormSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { z } from "zod";
import { Session } from "@/lib/types";

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
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    toast(await res.text());
  } catch (error) {
    console.error(error);
  }
}

export async function verify(token: string): Promise<string | null> {
  console.log(token);
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/auth/credential/verify?token=${token}`,
      {
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
      },
    );
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    let session = await res.text();

    sessionStorage.setItem("session", session);

    window.location.href = "/";

    return session;
  } catch (error) {
    console.error(error);
    return null;
  }
}

export async function signin(
  values: z.infer<typeof SignInFormSchema>,
): Promise<string | null> {
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

    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    let session = await res.text();

    sessionStorage.setItem("session", session);

    const url = window.location.href;
    window.location.href = url;
    if (url.includes("#")) window.location.reload();

    return session;
  } catch (error) {
    console.error(error);
    return null;
  }
}

export async function get_session(): Promise<Session | null> {
  try {
    const res = await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/me`, {
      headers: {
        Authorization: `Bearer ${sessionStorage.getItem("session")}`,
        "Content-Type": "application/json",
      },
    });
    if (!res.ok) {
      toast.error(await res.text());
      return null;
    }
    return await res.json();
  } catch (error) {
    console.error(error);
    return null;
  }
}

export async function refresh(): Promise<string | null> {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/auth/refresh`,
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
      },
    );
    if (!res.ok) {
      return null;
    }
    let session = await res.text();
    sessionStorage.setItem("session", session);
    return session;
  } catch (error) {
    console.error(error);
    return null;
  }
}

export async function signout() {
  try {
    await fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/auth/signout`, {
      credentials: "include",
    });

    sessionStorage.removeItem("session");

    const url = window.location.href;
    window.location.href = url;
    if (url.includes("#")) window.location.reload();
  } catch (error) {
    console.error(error);
  }
}
