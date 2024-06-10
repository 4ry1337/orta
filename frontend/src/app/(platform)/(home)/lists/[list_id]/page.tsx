"use client";

import { get_list } from "@/app/actions/list";
import ArticleTab from "@/components/article/article_tab";
import Aside from "@/components/aside";
import { ListComments } from "@/components/comment/comment_tab";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { get_id } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import React from "react";
import useSWR from "swr";

interface IParams {
  list_id: string;
}

const ListPage = ({ params }: { params: IParams }) => {
  const { data: list } = useSWR(get_id(params.list_id), get_list);

  if (!list) return <Skeleton className="w-full h-full" />;

  return (
    <div className="flex">
      <div className="w-full">
        <div className="p-4 flex flex-row items-center justify-between">
          <div className="">
            <h4>{list.label}</h4>
            <small className="text-muted-foreground">
              {list.article_count} Articles
            </small>
          </div>
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
        <Separator className="w-full" orientation="horizontal" />
        <div className="p-4 space-y-4">
          <ArticleTab list_id={list.id} />
        </div>
        <div className="block lg:hidden p-4 max-w-lg mx-auto space-y-4">
          <ListComments list_id={list.id} />
        </div>
      </div>
      <Aside className="space-y-4">
        <div className="flex flex-col">
          <h3>{list.label}</h3>
          <div className="inline-flex items-center gap-4">
            <h4 className="text-muted-foreground">
              {list.article_count} Articles
            </h4>
            <Button variant={"ghost"} size={"icon"}>
              <Share1Icon />
            </Button>
          </div>
        </div>
        <ListComments list_id={list.id} />
      </Aside>
    </div>
  );
};

export default ListPage;
