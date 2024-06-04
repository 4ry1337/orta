"use client";

import { HocuspocusProvider } from "@hocuspocus/provider";
import useSWR from "swr";
import { redirect } from "next/navigation";
import { toast } from "sonner";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import { get_article } from "@/app/actions/article";
import { Skeleton } from "@/components/ui/skeleton";
import ArticleSettingsTab from "./settings_tab";
import { get_id } from "@/lib/utils";
import EditorTab from "./editor_tab";
import { useMemo } from "react";
import PreviewTab from "./preview_tab";

interface IParams {
  article_id: string;
}

const ArticleEditPage = ({ params }: { params: IParams }) => {
  const { status, data } = useSession({
    authenticated: true,
  });

  const { data: article } = useSWR(get_id(params.article_id), get_article);

  const provider = useMemo(() => {
    return new HocuspocusProvider({
      url: "ws://127.0.0.1:6565",
      name: get_id(params.article_id),
      token: sessionStorage.getItem("session"),
    });
  }, [params.article_id]);

  if (status == "authenticated" && article) {
    if (!article.users?.map((user) => user.id).includes(data.user_id)) {
      toast.error("Forbidden");
      redirect("/");
    }

    return (
      <Tabs defaultValue={"editor"}>
        <div className="pt-6 px-3">
          <TabsList>
            <TabsTrigger value="editor">Editor</TabsTrigger>
            <TabsTrigger value="preview">Preview</TabsTrigger>
            <TabsTrigger value="article">Article</TabsTrigger>
          </TabsList>
        </div>
        <TabsContent value="editor">
          <EditorTab username={data.username} provider={provider} />
        </TabsContent>
        <TabsContent value="preview">
          <PreviewTab provider={provider} />
        </TabsContent>
        <TabsContent value="article">
          <ArticleSettingsTab article={article} />
        </TabsContent>
      </Tabs>
    );
  }

  return <Skeleton className="w-full min-h-screen" />;
};

export default ArticleEditPage;
