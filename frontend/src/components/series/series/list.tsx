import { Series } from "@/lib/types";
import SeriesCard from "./item";
import { HTMLAttributes } from "react";

interface SeriesListProps extends HTMLAttributes<HTMLDivElement> {
  serieses?: Series[];
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const SeriesList = ({ serieses, ...props }: SeriesListProps) => {
  if (!serieses) {
    return null;
  }
  return (
    <>
      {serieses.map((series) => (
        <SeriesCard {...props} key={series.id} series={series} />
      ))}
    </>
  );
};

export default SeriesList;
