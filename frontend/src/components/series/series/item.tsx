import { Button, buttonVariants } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { List, Series } from "@/lib/types";
import { DisplayDate } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import Image from "next/image";
import Link from "next/link";
import { HTMLAttributes } from "react";

interface SeriesCardProps extends HTMLAttributes<HTMLDivElement> {
  series: Series;
}

const SeriesCard = ({ series }: SeriesCardProps) => {
  return (
    <Card className="grid grid-cols-2 overflow-clip">
      <Link href={`/series/${series.slug}`}>
        <CardHeader>
          <CardTitle>{series.label}</CardTitle>
        </CardHeader>
        <CardContent>
          <small>{DisplayDate(series.created_at)}</small>
        </CardContent>
      </Link>
      <div className="relative">
        <Image
          alt="series-image"
          placeholder="empty"
          fill
          className="object-cover"
          src={`${series.image ?? "/placeholder.svg"}`}
        />
        <h1 className="absolute inset-0 bg-black/50 flex items-center justify-center text-muted-foreground">
          {series.article_count}
        </h1>
        <div className="absolute bottom-2 right-2">
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
            <span className="sr-only">Share</span>
          </Button>
        </div>
      </div>
    </Card>
  );
};

export default SeriesCard;
