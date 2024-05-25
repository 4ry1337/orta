"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import useSWR from "swr";
import { get_article } from "@/app/actions/article";
import ArticleCard from "@/components/article/list/item";
import { Skeleton } from "@/components/ui/skeleton";
import ArticleSettingsTab from "./settings_tab";
import { get_id } from "@/lib/utils";

interface IParams {
  article_id: string;
}

const ArticleEditPage = ({ params }: { params: IParams }) => {
  useSession({
    authenticated: true,
  });

  const { data: article, isLoading } = useSWR(
    get_id(params.article_id),
    get_article,
    {
      onSuccess(data, key, config) {
        console.log(data);
      },
    },
  );

  if (article)
    return (
      <Tabs defaultValue={"editor"} className="h-full">
        <TabsList>
          <TabsTrigger value="editor">Editor</TabsTrigger>
          <TabsTrigger value="preview">Preview</TabsTrigger>
          <TabsTrigger value="article">Article</TabsTrigger>
        </TabsList>
        <TabsContent value="editor">
          <ArticleCard article={article} />
          <div>Hello world</div>
        </TabsContent>
        <TabsContent value="preview">
          <div>Preview</div>
        </TabsContent>
        <TabsContent value="article">
          <ArticleSettingsTab article={article} />
        </TabsContent>
      </Tabs>
    );

  return <Skeleton className="w-full min-h-screen" />;
};

export default ArticleEditPage;
