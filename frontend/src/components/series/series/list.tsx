import { Series } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import SeriesCard from "./item";

interface SeriesListProps extends HTMLAttributes<HTMLDivElement> {
  serieses: Series[];
}

const SeriesList = ({ className, serieses }: SeriesListProps) => {
  return (
    <div className={cn("flex flex-col gap-4", className)}>
      {serieses.map((series) => (
        <SeriesCard key={series.id} series={series} />
      ))}
    </div>
  );
};

export default SeriesList;
