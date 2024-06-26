"use client";

import { get_following } from "@/app/actions/user";
import Link from "next/link";
import React, { useState } from "react";
import { Separator } from "@/components/ui/separator";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { FullUser } from "@/lib/types";
import UserList from "@/components/user/list/list";

interface IParams {
  username: string;
}

const FollowingPage = ({ params }: { params: IParams }) => {
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
      get_following(params.username, {
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
    <div className="w-full">
      <div className="p-4">
        <Link href={`/${params.username}`}>
          <h1>{params.username} follows:</h1>
        </Link>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4">
        {users.length == 0 ? (
          <div className="h-96 content-center">
            <h1 className="text-center">No follows</h1>
          </div>
        ) : (
          <UserList users={users} />
        )}
        {(isLoading || hasNextPage) && (
          <div className="w-full h-20" ref={ref} />
        )}
      </div>
    </div>
  );
};

export default FollowingPage;
