"use client";

import { follow, get_user, unfollow } from "@/app/actions/user";
import Aside from "@/components/aside";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import { Share1Icon } from "@radix-ui/react-icons";
import { VerifiedIcon } from "lucide-react";
import Link from "next/link";
import React from "react";
import useSWR from "swr";
import { Separator } from "@/components/ui/separator";
import ArticleTab from "@/components/article/article_tab";
import SeriesTab from "@/components/series/series_tab";
import ListTab from "@/components/list/list_tab";

interface IParams {
  username: string;
}

const User = ({ params }: { params: IParams }) => {
  const { data } = useSession();

  const { data: user, mutate } = useSWR(params.username, get_user);

  if (!user) return <Skeleton className="w-full h-full" />;

  return (
    <div className="flex">
      <div className="w-full">
        <div className="hidden p-4 lg:block">
          <Link href={`/${user.username}`}>
            <h1>{user.username}</h1>
          </Link>
        </div>
        <div className="lg:hidden p-4 flex items-center flex-row gap-8 justify-between">
          <div className="flex flex-row gap-4">
            <Avatar className="w-20 h-20">
              <AvatarImage
                src={
                  user.image && "http://localhost:5000/api/assets/" + user.image
                }
                className="object-cover"
                alt="@avatar"
              />
              <AvatarFallback>{user.username.at(0)}</AvatarFallback>
            </Avatar>
            <div className="flex flex-col">
              <div className="flex flex-row gap-1">
                <Link href={`/${user.username}`}>
                  <h1>{user.username}</h1>
                </Link>
                {user.approved_at ? <VerifiedIcon /> : null}
              </div>
              <div>
                <Link href={`/${user.username}/followers`}>
                  <small className="mr-2">
                    {user.follower_count} Folllowers
                  </small>
                </Link>
                <Link href={`/${user.username}/following`}>
                  <small>{user.following_count} Folllowing</small>
                </Link>
              </div>
            </div>
          </div>
          <div className="flex flex-row gap-4">
            {data?.user_id != user.id ? (
              user.followed ? (
                <Button
                  onClick={() => {
                    unfollow(user.username);
                    mutate({
                      ...user,
                      followed: false,
                      follower_count: user.follower_count - 1,
                    });
                  }}
                >
                  Unfollow
                </Button>
              ) : (
                <Button
                  onClick={() => {
                    follow(user.username);
                    mutate({
                      ...user,
                      followed: true,
                      follower_count: user.follower_count + 1,
                    });
                  }}
                >
                  Follow
                </Button>
              )
            ) : (
              <Button asChild>
                <Link href={"/settings"}>Edit Profile</Link>
              </Button>
            )}
            <Button variant={"ghost"} size={"icon"}>
              <Share1Icon />
            </Button>
          </div>
        </div>
        <Separator className="w-full" orientation="horizontal" />
        <div className="p-4">
          <Tabs defaultValue={"article"}>
            <TabsList>
              <TabsTrigger asChild value="article">
                <Link href={"#article"}>Article</Link>
              </TabsTrigger>
              <TabsTrigger value="series">
                <Link href={"#series"}>Series</Link>
              </TabsTrigger>
              <TabsTrigger value="lists">
                <Link href={"#lists"}>Lists</Link>
              </TabsTrigger>
            </TabsList>
            <TabsContent className="mt-4 space-y-4" value="article">
              <ArticleTab username={user.username} />
            </TabsContent>
            <TabsContent className="mt-4 space-y-4" value="series">
              <SeriesTab username={user.username} />
            </TabsContent>
            <TabsContent className="mt-4 space-y-4" value="lists">
              <ListTab username={user.username} />
            </TabsContent>
          </Tabs>
        </div>
      </div>
      <Aside>
        <div className="space-y-4">
          <div className="flex flex-row items-center gap-4">
            <Avatar className="w-20 h-20">
              <AvatarImage
                src={
                  user.image && "http://localhost:5000/api/assets/" + user.image
                }
                className="object-cover"
                alt="@avatar"
              />
              <AvatarFallback>{user.username.at(0)}</AvatarFallback>
            </Avatar>
            <div className="flex flex-col">
              <div className="flex flex-row gap-1">
                <Link href={`/${user.username}`}>
                  <h3>{user.username}</h3>
                </Link>
                {user.approved_at ? <VerifiedIcon /> : null}
              </div>
            </div>
          </div>
          <div>
            <Link href={`/${user.username}/followers`}>
              <small className="mr-2">{user.follower_count} Folllowers</small>
            </Link>
            <Link href={`/${user.username}/following`}>
              <small>{user.following_count} Folllowing</small>
            </Link>
          </div>
          <p className="">{user.bio ?? ""}</p>
          <div className="flex flex-row gap-4">
            {data?.user_id != user.id ? (
              user.followed ? (
                <Button
                  onClick={() => {
                    unfollow(user.username);
                    mutate();
                  }}
                >
                  Unfollow
                </Button>
              ) : (
                <Button
                  onClick={() => {
                    follow(user.username);
                    mutate();
                  }}
                >
                  Follow
                </Button>
              )
            ) : (
              <Button asChild>
                <Link href={"/settings"}>Edit Profile</Link>
              </Button>
            )}
            <Button variant={"ghost"} size={"icon"}>
              <Share1Icon />
            </Button>
          </div>
        </div>
      </Aside>
    </div>
  );
};

// <div className="flex flex-col items-start">
//   {user.urls.map((url) => (
//     <Button asChild key={url} variant={"link"}>
//       <Link href={url}>{url}</Link>
//     </Button>
//   ))}
// </div>

export default User;
