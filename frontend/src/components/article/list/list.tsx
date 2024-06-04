"use client";

import { FullArticle } from "@/lib/types";
import { HTMLAttributes } from "react";
import ArticleCard from "./item";

interface ArticleListProps extends HTMLAttributes<HTMLDivElement> {
  articles?: FullArticle[];
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const ArticleList = ({ articles, ...props }: ArticleListProps) => {
  if (!articles) {
    return null;
  }
  return (
    <>
      {articles.map((article) => (
        <ArticleCard {...props} key={article.id} article={article} />
      ))}
    </>
  );
};

export default ArticleList;
