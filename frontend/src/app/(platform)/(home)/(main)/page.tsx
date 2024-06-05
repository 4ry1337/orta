"use client";

import ArticleTab from "@/components/article/article_tab";
import { Separator } from "@/components/ui/separator";

const Home = () => {
  return (
    <div className="w-full">
      <div className="p-4 space-y-4">
        <ArticleTab />
      </div>
    </div>
  );
};

export default Home;
