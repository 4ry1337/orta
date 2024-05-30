"use client";

import { get_session, refresh } from "@/app/actions/auth";
import useSWR from "swr";
import { createContext, useContext, useMemo, useState } from "react";
import { redirect } from "next/navigation";
import { Session } from "@/lib/types";

export type SessionStatus = "loading" | "authenticated" | "unauthenticated";

export type UpdateSession = () => Promise<Session | null>;

export type SessionContextValue<R extends boolean | undefined = undefined> =
  R extends true
  ?
  | { update: UpdateSession; data: Session; status: "authenticated" }
  | { update: UpdateSession; data: null; status: "loading" }
  : R extends false
  ?
  | { update: UpdateSession; data: Session; status: "unauthenticated" }
  | { update: UpdateSession; data: null; status: "loading" }
  :
  | {
    update: UpdateSession;
    data: Session;
    status: "authenticated";
  }
  | {
    update: UpdateSession;
    data: null;
    status: "unauthenticated" | "loading";
  };

export const SessionContext = createContext<SessionContextValue | undefined>(
  undefined,
);

export interface UseSessionOptions<R extends boolean | undefined> {
  authenticated: R;
  logger?: boolean;
  onUnauthenticated?: () => void;
  onAuthenticated?: () => void;
}

export function useSession<R extends boolean | undefined>(
  options?: UseSessionOptions<R>,
): SessionContextValue<R> {
  if (!SessionContext) {
    throw new Error("React Context is unavailable in Server Components");
  }

  const value:
    | {
      update: UpdateSession;
      data: Session | null;
      status: "authenticated" | "unauthenticated" | "loading";
    }
    | undefined = useContext(SessionContext);

  if (!value) {
    throw new Error(
      "[auth]: `useSession` must be wrapped in a <SessionProvider />",
    );
  }

  const { logger, authenticated, onUnauthenticated, onAuthenticated } =
    options ?? {};

  const notrequiredAndNotLoading =
    authenticated === false && value.status === "authenticated";

  const requiredAndNotLoading =
    authenticated === true && value.status === "unauthenticated";

  if (logger) {
    console.log(
      "notrequiredAndNotLoading > ",
      authenticated === false && value.status === "authenticated",
      authenticated === false,
      value.status === "authenticated",
    );
    console.log(
      "requiredAndNotLoading > ",
      authenticated === true && value.status === "unauthenticated",
      authenticated === true,
      value.status === "unauthenticated",
    );
  }

  if (requiredAndNotLoading) {
    if (logger) console.log("redirect to auth");
    if (onUnauthenticated) onUnauthenticated();
    else redirect("/auth");
  }
  if (notrequiredAndNotLoading) {
    if (logger) console.log("redirect to main");
    if (onAuthenticated) onAuthenticated();
    else redirect("/");
  }

  if (logger) console.log("useSession > ", value);

  return value as SessionContextValue<R>;
}

export type SessionProviderProps = {
  children: React.ReactNode;
};

const SessionProvider = (props: SessionProviderProps) => {
  if (!SessionContext) {
    throw new Error("React Context is unavailable in Server Components");
  }

  const [loading, setLoading] = useState(true);

  useSWR("session", refresh, {
    refreshInterval: 5 * 60 * 1000,
    onSuccess(data, key, config) {
      setToken(data);
      if (data === null) {
        setLoading(false);
      }
    },
  });

  const { children } = props;

  const [token, setToken] = useState<string | null | undefined>(undefined);

  const [session, setSession] = useState<Session | null>(null);

  const session_res = useSWR(token, get_session, {
    onSuccess(data, key, config) {
      if (data != session) {
        setSession(data);
      }
      setLoading(false);
    },
  });

  let value:
    | {
      update: UpdateSession;
      data: Session;
      status: "authenticated";
    }
    | {
      update: UpdateSession;
      data: null;
      status: "unauthenticated" | "loading";
    }
    | undefined = useMemo(() => {
      return session
        ? {
          data: session,
          status: "authenticated",
          async update() {
            setLoading(true);
            const newSession = await get_session();
            setLoading(false);
            if (newSession) {
              setSession(newSession);
            }
            return newSession;
          },
        }
        : {
          data: session,
          status: loading ? "loading" : "unauthenticated",
          async update() {
            return null;
          },
        };
    }, [session, loading]);

  return (
    <SessionContext.Provider value={value}>{children}</SessionContext.Provider>
  );
};

export default SessionProvider;
