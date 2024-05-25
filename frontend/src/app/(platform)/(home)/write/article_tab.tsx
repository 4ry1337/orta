"use client";

import { get_articles } from "@/app/actions/article";
import CreateArticleDialog from "@/components/article/create_article_dialog";
import ArticleList from "@/components/article/list/list";
import { Skeleton } from "@/components/ui/skeleton";
import { HTMLAttributes, useState } from "react";
import useSWR from "swr";

interface ArticleTabProps extends HTMLAttributes<HTMLDivElement> {
  username: string;
}

const ArticleTab = ({ username }: ArticleTabProps) => {
  const [page, setPage] = useState(1);

  const { data: articles } = useSWR(
    {
      usernames: [username],
      pagination: {
        page: page,
        per_page: 10,
      },
    },
    get_articles,
  );

  return (
    <div className="space-y-4">
      <CreateArticleDialog />
      {!articles && <Skeleton className="h-60 w-full" />}
      {!!articles && <ArticleList articles={articles.items} />}
    </div>
  );
};

export default ArticleTab;
