"use client";

import { get_articles } from "@/app/actions/article";
import {
  add_article_series,
  get_series,
  remove_article_series,
} from "@/app/actions/series";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { FullArticle } from "@/lib/types";
import { get_id } from "@/lib/utils";
import { useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import useSWR from "swr";
import { DndContext, DragEndEvent } from "@dnd-kit/core";
import ArticleDroppable from "./article_droppable";

interface IParams {
  series_id: string;
}

const SeriesEditPage = ({ params }: { params: IParams }) => {
  const { data: session, status } = useSession({
    authenticated: true,
  });

  const { data: series } = useSWR(get_id(params.series_id), get_series);

  const [limit, setLimit] = useState(5);

  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasNextPage, setHasNextPage] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      if (!session) return;
      get_articles({
        username: session.username,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setArticles([
          ...articles,
          ...data.items.filter((article) => article.series.length === 0),
        ]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  const [seriesArticles, setSeriesArticles] = useState<FullArticle[]>([]);

  const [seriesCursor, setSeriesCursor] = useState<string | undefined>(
    undefined,
  );

  const [hasSeriesNextPage, setHasSeriesNextPage] = useState(true);

  const [isSeriesLoading, setIsSeriesLoading] = useState(false);

  const [seriesRef] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      if (!series || !session) return;
      get_articles({
        username: session.username,
        series_id: series.id,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setSeriesArticles([...articles, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  function addArticleToSeries(event: DragEndEvent) {
    const article = event.active.data.current as FullArticle;
    const { over } = event;
    if (!series || !article) return;

    if (over?.id === series.id && article.series.length == 0) {
      setArticles(articles.filter((a) => a != article));
      setSeriesArticles([...seriesArticles, { ...article, series: [series] }]);
      add_article_series(series.id, article.id);
      console.log(seriesArticles);
    }

    if (over?.id === "no_series" && article.series.length != 0) {
      setSeriesArticles(seriesArticles.filter((a) => a != article));
      setArticles([...articles, { ...article, series: [series] }]);
      remove_article_series(series.id, article.id);
    }
  }

  if (!series || status == "loading")
    return <Skeleton className="w-full h-full" />;

  return (
    <div className="w-full">
      <div className="p-4 flex flex-row justify-between items-center">
        <div className="">
          <h3>{series.label}</h3>
          <div className="text-muted-foreground">
            {series.article_count} Articles
          </div>
        </div>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4 grid grid-cols-2 gap-2">
        <DndContext onDragEnd={addArticleToSeries}>
          <div className="flex flex-col h-full">
            <div className="py-2">
              <h4>In Series:</h4>
            </div>
            <ArticleDroppable id={series.id} articles={seriesArticles} />
            {(isSeriesLoading || hasSeriesNextPage) && (
              <div className="w-full h-20" ref={seriesRef} />
            )}
          </div>
          <div className="flex flex-col h-full">
            <div className="py-2">
              <h4>No Series:</h4>
            </div>
            <ArticleDroppable id={"no_series"} articles={articles} />
            {(isLoading || hasNextPage) && (
              <div className="w-full h-20" ref={ref} />
            )}
          </div>
        </DndContext>
      </div>
    </div>
  );
};

export default SeriesEditPage;
