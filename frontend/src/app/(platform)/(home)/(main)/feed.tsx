"use client";

import ArticleList from "@/components/article/list/list";
import { FullArticle } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { HTMLAttributes, useState } from "react";
import { get_feed } from "@/app/actions/user";

const Feed = () => {
  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [limit, setLimit] = useState(10);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_feed({
        cursor,
        limit,
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
      {articles.length != 0 ? (
        <ArticleList articles={articles} />
      ) : (
        <h4 className="text-center">No Articles</h4>
      )}
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </>
  );
};

export default Feed;
