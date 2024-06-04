"use client";

import { get_series } from "@/app/actions/series";
import ArticleTab from "@/components/article/article_tab";
import Aside from "@/components/aside";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { get_id } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import Link from "next/link";
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
    <div className="flex">
      <div className="w-full">
        <div className="p-4">
          <h1>{series.label}</h1>
        </div>
        <Separator className="w-full" orientation="horizontal" />
        <div className="p-4 space-y-4">
          <ArticleTab serieses={[series.id]} />
        </div>
      </div>
      <Aside>
        <div className="space-y-4">
          <div className="flex">
            <h3>{series.label}</h3>
          </div>
          <h4 className="text-muted-foreground">
            {series.article_count} Articles
          </h4>
          <div className="flex flex-row gap-4">
            {session?.user_id !== series.id && (
              <Button asChild>
                <Link href={"/settings"}>Edit Profile</Link>
              </Button>
            )}
            <Button variant={"ghost"} size={"icon"}>
              <Share1Icon />
            </Button>
          </div>
          <div></div>
        </div>
      </Aside>
    </div>
  );
};

export default SeriesPage;
