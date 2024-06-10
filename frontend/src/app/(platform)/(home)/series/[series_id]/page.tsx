"use client";

import { get_series } from "@/app/actions/series";
import ArticleTab from "@/components/article/article_tab";
import Aside from "@/components/aside";
import { SeriesComments } from "@/components/comment/comment_tab";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { get_id } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import React from "react";
import useSWR from "swr";

interface IParams {
  series_id: string;
}

const SeriesPage = ({ params }: { params: IParams }) => {
  const { data: series } = useSWR(get_id(params.series_id), get_series);

  if (!series) return <Skeleton className="w-full h-full" />;

  return (
    <div className="flex">
      <div className="w-full">
        <div className="p-4 flex flex-row items-center justify-between">
          <div className="">
            <h4>{series.label}</h4>
            <small className="text-muted-foreground">
              {series.article_count} Articles
            </small>
          </div>
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
        <Separator className="w-full" orientation="horizontal" />
        <div className="p-4 space-y-4">
          <ArticleTab series_id={series.id} />
        </div>
        <div className="block lg:hidden p-4 max-w-lg mx-auto space-y-4">
          <SeriesComments series_id={series.id} />
        </div>
      </div>
      <Aside className="space-y-4">
        <div className="flex">
          <div className="inline-flex items-center gap-4">
            <h3>{series.label}</h3>
            <h4 className="text-muted-foreground">
              {series.article_count} Articles
            </h4>
          </div>
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
        <div className="grow">
          <SeriesComments series_id={series.id} />
        </div>
      </Aside>
    </div>
  );
};

export default SeriesPage;
