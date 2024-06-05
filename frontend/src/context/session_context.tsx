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
  | {
    update: UpdateSession;
    data: Session;
    status: "authenticated";
  }
  | { update: UpdateSession; data: null; status: "loading" }
  : R extends false
  ?
  | {
    update: UpdateSession;
    data: Session;
    status: "unauthenticated";
  }
  | {
    update: UpdateSession;
    data: null;
    status: "loading";
  }
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

  const { authenticated, onUnauthenticated, onAuthenticated } = options ?? {};

  const notrequiredAndNotLoading =
    authenticated === false && value.status === "authenticated";

  const requiredAndNotLoading =
    authenticated === true && value.status === "unauthenticated";

  if (requiredAndNotLoading) {
    if (onUnauthenticated) onUnauthenticated();
    else redirect("/auth");
  }
  if (notrequiredAndNotLoading) {
    if (onAuthenticated) onAuthenticated();
    else redirect("/");
  }

  return value as SessionContextValue<R>;
}

// export const TokenContext = createContext<{ token: string | null | undefined }>(
//   {
//     token: undefined,
//   },
// );
//
// export const useToken = () => {
//   if (!TokenContext) {
//     throw new Error("React Context is unavailable in Server Components");
//   }
//
//   const value = useContext(TokenContext);
//
//   if (!value) {
//     throw new Error(
//       "[token]: `useToken` must be wrapped in a <TokenProvider />",
//     );
//   }
//
//   return value;
// };

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

  const [loading, setLoading] = useState(true);

  const { mutate } = useSWR("token", refresh, {
    onSuccess(data) {
      setToken(data);
      if (data === null) {
        setLoading(false);
      }
    },
    refreshInterval: 5 * 60 * 1000,
    revalidateOnReconnect: false,
    revalidateIfStale: false,
    revalidateOnFocus: false,
  });

  useSWR(token, get_session, {
    onSuccess(data) {
      if (data != session) {
        setSession(data);
      }
      setLoading(false);
    },
    revalidateOnReconnect: false,
    revalidateIfStale: false,
    revalidateOnFocus: false,
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
            await mutate();
            return session;
          },
        }
        : {
          data: session,
          status: loading ? "loading" : "unauthenticated",
          async update() {
            return null;
          },
        };
    }, [session, loading, mutate]);

  return (
    <SessionContext.Provider value={value}>{children}</SessionContext.Provider>
  );
};

export default SessionProvider;
