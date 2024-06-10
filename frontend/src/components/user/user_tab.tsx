import React, { useState } from "react";
import UserList from "./list/list";
import { FullUser } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { get_users } from "@/app/actions/user";

const UserTab = () => {
  const [users, setUsers] = useState<FullUser[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasNextPage, setHasNextPage] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      get_users(undefined, {
        cursor,
        limit,
      }).then((data) => {
        setUsers([...users, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  return (
    <div className="flex flex-col gap-4">
      <UserList badge={false} users={users} />
      {(isLoading || hasNextPage) && <div className="w-full h-20" ref={ref} />}
    </div>
  );
};

export default UserTab;
