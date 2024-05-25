"use client";

import { get_series } from "@/app/actions/series";
import CreateSeriesDialog from "@/components/series/create_series_dialog";
import SeriesList from "@/components/series/series/list";
import { Skeleton } from "@/components/ui/skeleton";
import { HTMLAttributes, useState } from "react";
import useSWR from "swr";

interface SeriesTabProps extends HTMLAttributes<HTMLDivElement> {
  user_id: string;
}

const SeriesTab = ({ user_id }: SeriesTabProps) => {
  const [page, setPage] = useState(1);

  const { data: series } = useSWR(
    {
      user_id: user_id,
      pagination: {
        page: page,
        per_page: 10,
      },
    },
    get_series,
  );
  return (
    <div className="space-y-4">
      <CreateSeriesDialog />
      {!series && <Skeleton className="h-40 w-full" />}
      {!!series && (
        <SeriesList
          className="grid grid-cols-2 lg:grid-cols-3"
          serieses={series.items}
        />
      )}
    </div>
  );
};

export default SeriesTab;
