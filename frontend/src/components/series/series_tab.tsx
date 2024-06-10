"use client";

import { Series } from "@/lib/types";
import { useState } from "react";
import SeriesList from "./series/list";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { get_user_serieses } from "@/app/actions/user";

const SeriesTab = ({ username }: { username: string }) => {
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
      get_user_serieses(username, {
        cursor,
        limit,
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
    <>
      {serieses.length != 0 && !hasMore ? (
        <div className="grid grid-cols-2 gap-4">
          <SeriesList serieses={serieses} />
        </div>
      ) : (
        <h4 className="text-center">No Series</h4>
      )}
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </>
  );
};

export default SeriesTab;
