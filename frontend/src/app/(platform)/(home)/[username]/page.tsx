"use client";

import { get_user } from "@/app/actions/user";
import { Skeleton } from "@/components/ui/skeleton";
import React from "react";
import useSWR from "swr";

interface IParams {
  username: string;
}

const User = ({ params }: { params: IParams }) => {
  const { data, error } = useSWR(params.username, get_user, {
    onError(err, key, config) {
      console.log(err);
    },
  });

  if (error)
    return (
      <div className="h-full w-full content-center">
        <h1 className="text-center">{error.message}</h1>
      </div>
    );

  if (!data) return <Skeleton className="w-full h-full" />;

  return (
    <div className="px-4 py-2">
      <h1>{data.username}</h1>
      <h1>{data.email}</h1>
      <h1>{data.bio}</h1>
      <h1>{data.follower_count}</h1>
      <h1>{data.following_count}</h1>
    </div>
  );
};

export default User;
