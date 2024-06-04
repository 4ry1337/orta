"use client";

import { List } from "@/lib/types";
import { HTMLAttributes, useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import ListList from "./list/list";
import { get_lists } from "@/app/actions/list";

interface ListTabProps extends HTMLAttributes<HTMLDivElement> {
  user_id?: string;
  query?: string;
}

const ListTab = ({ user_id, query: label }: ListTabProps) => {
  const [lists, setLists] = useState<List[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_lists({
        user_id,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setLists([...lists, ...data.items]);
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
    <div className="flex flex-col gap-4">
      <ListList lists={lists} />
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </div>
  );
};

export default ListTab;
