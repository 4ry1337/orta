"use client";

import { get_session, refresh } from "@/app/actions/auth";
import useSWR from "swr";
import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { redirect } from "next/navigation";
import { Session } from "@/lib/types";
import { toast } from "sonner";

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
    status: "unauthenticated";
  }
  | {
    update: UpdateSession;
    data: null;
    status: "loading";
  };

export const SessionContext = createContext<SessionContextValue | undefined>(
  undefined,
);

export interface UseSessionOptions<R extends boolean | undefined> {
  authenticated: R;
  /** Defaults to `auth` */
  onUnauthenticated?: () => void;
  /** Defaults to `home` */
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

  const { authenticated, onUnauthenticated, onAuthenticated } = options ?? {};

  const notrequiredAndNotLoading =
    authenticated === false && value.status === "authenticated";

  const requiredAndNotLoading =
    authenticated === true && value.status === "unauthenticated";

  useEffect(() => {
    if (notrequiredAndNotLoading) {
      if (onAuthenticated) onAuthenticated();
      else redirect("/");
    }
    if (requiredAndNotLoading) {
      if (onUnauthenticated) onUnauthenticated();
      else redirect("/auth");
    }
  }, [
    requiredAndNotLoading,
    onUnauthenticated,
    notrequiredAndNotLoading,
    onAuthenticated,
  ]);

  if (notrequiredAndNotLoading || requiredAndNotLoading) {
    return {
      data: value.data,
      update: value.update,
      status: "loading",
    } as SessionContextValue<R>;
  }

  return value as SessionContextValue<R>;
}

export type SessionProviderProps = {
  children: React.ReactNode;
};

const SessionProvider = (props: SessionProviderProps) => {
  if (!SessionContext) {
    throw new Error("React Context is unavailable in Server Components");
  }

  const { children } = props;

  const [token, setToken] = useState<string | null | undefined>(undefined);

  const [session, setSession] = useState<Session | null>(null);

  const token_res = useSWR("session", refresh, {
    refreshInterval: 5 * 60 * 1000,
    onSuccess(data, key, config) {
      if (data !== token) {
        setToken(data);
      }
    },
  });

  const session_res = useSWR(token, get_session, {
    onSuccess(data, key, config) {
      if (data != session) {
        setSession(data);
      }
    },
  });

  const [loading, setLoading] = useState(false);

  let value:
    | {
      update: UpdateSession;
      data: Session;
      status: "authenticated";
    }
    | {
      update: UpdateSession;
      data: null;
      status: "unauthenticated";
    }
    | {
      update: UpdateSession;
      data: null;
      status: "loading";
    }
    | undefined = useMemo(
      () =>
        loading
          ? {
            data: null,
            status: "loading",
            async update() {
              return null;
            },
          }
          : session !== null
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
              status: "unauthenticated",
              async update() {
                toast.error("Not authenticated");
                return null;
              },
            },
      [session, token],
    );

  return (
    <SessionContext.Provider value={value}>{children}</SessionContext.Provider>
  );
};

export default SessionProvider;
