"use client";

import { get_articles } from "@/app/actions/article";
import { get_series } from "@/app/actions/series";
import ArticleList from "@/components/article/list/list";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { get_id } from "@/lib/utils";
import { useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import useSWR from "swr";

interface IParams {
  series_id: string;
}

const SeriesEditPage = ({ params }: { params: IParams }) => {
  const { data: session } = useSession({
    authenticated: true,
  });

  const { data: series } = useSWR(get_id(params.series_id), get_series);

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
        usernames: [session!.username],
        serieses: [series!.id],
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

  if (!series) return <Skeleton className="w-full h-full" />;

  return (
    <div className="w-full">
      <div className="p-4">
        <h1>{series.label}</h1>
        <h4 className="text-muted-foreground">
          {series.article_count} Articles
        </h4>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4 space-y-4">
        <ArticleList articles={articles} />
        {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
      </div>
    </div>
  );
};

export default SeriesEditPage;
