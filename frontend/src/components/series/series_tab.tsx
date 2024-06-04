"use client";

import { Series } from "@/lib/types";
import { HTMLAttributes, useState } from "react";
import useSWR from "swr";
import SeriesList from "./series/list";
import { get_serieses } from "@/app/actions/series";
import useInfiniteScroll from "react-infinite-scroll-hook";

interface SeriesTabProps extends HTMLAttributes<HTMLDivElement> {
  user_id?: string;
  query?: string;
}

const SeriesTab = ({ user_id, query }: SeriesTabProps) => {
  const [serieses, setSerieses] = useState<Series[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_serieses({
        user_id,
        query,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setSerieses([...serieses, ...data.items]);
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
    <div className="grid grid-cols-2 gap-4">
      <SeriesList serieses={serieses} />
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </div>
  );
};

export default SeriesTab;
