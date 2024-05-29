"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import useSWR from "swr";
import { get_article } from "@/app/actions/article";
import { Skeleton } from "@/components/ui/skeleton";
import ArticleSettingsTab from "./settings_tab";
import { get_id } from "@/lib/utils";
import PreviewTab from "./preview_tab";
import EditorTab from "./editor_tab";
import { redirect } from "next/navigation";
import { toast } from "sonner";

interface IParams {
  article_id: string;
}

const ArticleEditPage = ({ params }: { params: IParams }) => {
  const { status, data } = useSession({
    authenticated: true,
  });

  const { data: article } = useSWR(get_id(params.article_id), get_article);

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
          <EditorTab username={data.username} article_id={article.id} />
        </TabsContent>
        <TabsContent value="preview">
          <PreviewTab article={article} />
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
