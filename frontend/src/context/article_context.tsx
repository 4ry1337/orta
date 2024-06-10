"use client";

import { HocuspocusProvider } from "@hocuspocus/provider";
import useSWR from "swr";
import { redirect } from "next/navigation";
import { toast } from "sonner";
import { useSession } from "@/context/session_context";
import { get_article } from "@/app/actions/article";
import { Skeleton } from "@/components/ui/skeleton";
import { createContext, useContext, useMemo, useState } from "react";
import { FullArticle, Session } from "@/lib/types";

export type ArticleContextValue = {
  provider: HocuspocusProvider;
  article: FullArticle;
  session: Session;
  update: (newArticle: FullArticle) => void;
};

export const ArticleContext = createContext<ArticleContextValue | undefined>(
  undefined,
);

export function useArticle() {
  if (!ArticleContext) {
    throw new Error("React Context is unavailable in Server Components");
  }
  const value = useContext(ArticleContext);

  if (!value) {
    throw new Error(
      "[auth]: `useArticle` must be wrapped in a <ArticleProvider />",
    );
  }

  return value;
}

export type ArticleProviderProps = {
  article_id: string;
  children: React.ReactNode;
};

export const ArticleProvider = (props: ArticleProviderProps) => {
  const { status, data: session } = useSession({
    authenticated: true,
  });

  const [article, setArticle] = useState<FullArticle | null>(null);

  useSWR(props.article_id, get_article, {
    onSuccess(data) {
      setArticle(data);
    },
  });

  if (status == "authenticated" && article) {
    if (!article.users?.map((user) => user.id).includes(session.user_id)) {
      toast.error("Forbidden");
      redirect("/");
    }
  }

  const provider = useMemo(() => {
    if (article?.id && session) {
      return new HocuspocusProvider({
        url: "ws://127.0.0.1:6565",
        name: article.id,
        token: sessionStorage.getItem("session"),
      });
    }
  }, [article?.id, session]);

  const value = useMemo(() => {
    if (article && provider && session) {
      return {
        article,
        session,
        provider,
        update: (newArticle: FullArticle) => {
          setArticle(newArticle);
        },
      };
    }
  }, [article, provider, session]);

  if (!value) return <Skeleton className="w-full min-h-screen" />;

  return (
    <ArticleContext.Provider value={value}>
      {props.children}
    </ArticleContext.Provider>
  );
};
