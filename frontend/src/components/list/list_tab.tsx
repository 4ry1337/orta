"use client";

import { List } from "@/lib/types";
import { useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import ListList from "./list/list";
import { get_user_lists } from "@/app/actions/user";

const ListTab = ({ username }: { username: string }) => {
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
      get_user_lists(username, {
        cursor,
        limit,
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
    <>
      {lists.length != 0 ? (
        <ListList lists={lists} />
      ) : (
        <h4 className="text-center">No Lists</h4>
      )}
      {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
    </>
  );
};

export default ListTab;
