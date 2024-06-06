"use client";

import { get_series } from "@/app/actions/series";
import ArticleTab from "@/components/article/article_tab";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { get_id } from "@/lib/utils";
import React from "react";
import useSWR from "swr";

interface IParams {
  series_id: string;
}

const SeriesPage = ({ params }: { params: IParams }) => {
  const { data: session } = useSession();

  const { data: series } = useSWR(get_id(params.series_id), get_series);

  if (!series) return <Skeleton className="w-full h-full" />;

  return (
    <div className="w-full">
      <div className="p-4">
        <h1>{series.label}</h1>
        <h4 className="text-muted-foreground">
          {series.article_count} Articles
        </h4>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4 space-y-4">
        <ArticleTab series_id={series.id} />
      </div>
    </div>
  );
};

export default SeriesPage;
