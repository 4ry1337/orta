"use client";

import ArticleTab from "@/components/article/article_tab";
import { useSession } from "@/context/session_context";
import Feed from "./feed";
import { Separator } from "@/components/ui/separator";

const Home = () => {
  const { status, data: session } = useSession();
  return (
    <div className="w-full">
      <div className="p-4 block">
        <h1>Feed</h1>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4 space-y-4">{status ? <Feed /> : <ArticleTab />}</div>
    </div>
  );
};

export default Home;
