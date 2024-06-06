"use client";

import { get_articles, get_user_articles } from "@/app/actions/article";
import ArticleList from "@/components/article/list/list";
import { FullArticle } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { HTMLAttributes, useState } from "react";

interface ArticleTabProps extends HTMLAttributes<HTMLDivElement> {
  query?: string;
  username?: string;
  list_id?: string;
  series_id?: string;
  published?: boolean;
}

const ArticleTab = ({
  query,
  published,
  username,
  series_id,
  list_id,
}: ArticleTabProps) => {
  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_articles({
        query,
        username,
        published,
        series_id,
        list_id,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setArticles([...articles, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasMore(false);
        }
      });
      setIsLoading(false);
    },
  });

  return (
    <>
      <ArticleList articles={articles} />
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </>
  );
};

export default ArticleTab;
