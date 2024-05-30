"use client";

import { FullArticle } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import ArticleCard from "./item";

interface ArticleListProps extends HTMLAttributes<HTMLDivElement> {
  articles?: FullArticle[];
}

const ArticleList = ({ articles, className }: ArticleListProps) => {
  if (!articles) {
    return null;
  }
  return (
    <div className={cn("flex flex-col gap-4", className)}>
      {articles.map((article) => (
        <ArticleCard key={article.id} article={article} />
      ))}
    </div>
  );
};

export default ArticleList;
