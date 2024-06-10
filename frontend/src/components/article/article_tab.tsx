"use client";

import ArticleList from "@/components/article/list/list";
import { FullArticle } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { HTMLAttributes, useState } from "react";
import { get_user_articles } from "@/app/actions/user";
import { get_list_articles } from "@/app/actions/list";
import { get_series_articles } from "@/app/actions/series";
import { get_articles } from "@/app/actions/article";

interface ArticleTabProps extends HTMLAttributes<HTMLDivElement> {
  query?: string;
  username?: string;
  list_id?: string;
  series_id?: string;
}

const ArticleTab = ({
  query,
  username,
  series_id,
  list_id,
}: ArticleTabProps) => {
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
      if (username) {
        get_user_articles(username, {
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
      } else if (list_id) {
        get_list_articles(list_id, {
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
      } else if (series_id) {
        get_series_articles(series_id, {
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
      } else {
        get_articles(query, {
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
      }

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

export default ArticleTab;
