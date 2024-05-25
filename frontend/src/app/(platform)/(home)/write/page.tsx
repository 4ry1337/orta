"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import ArticleTab from "./article_tab";
import { Skeleton } from "@/components/ui/skeleton";
import Link from "next/link";
import SeriesTab from "./series_tab";

const WritePage = () => {
  const { status, data } = useSession({
    authenticated: true,
  });

  if (status == "loading") {
    return <Skeleton className="h-screen w-full" />;
  }

  return (
    <Tabs defaultValue={"article"}>
      <div className="pt-6 px-2">
        <TabsList>
          <TabsTrigger asChild value="article">
            <Link href={"#article"}>Article</Link>
          </TabsTrigger>
          <TabsTrigger value="series">
            <Link href={"#series"}>Series</Link>
          </TabsTrigger>
        </TabsList>
      </div>
      <TabsContent className="px-2" value="article">
        <ArticleTab username={data.username} />
      </TabsContent>
      <TabsContent className="px-2" value="series">
        <SeriesTab user_id={data.user_id} />
      </TabsContent>
    </Tabs>
  );
};

export default WritePage;
