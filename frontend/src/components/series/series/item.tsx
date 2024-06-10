import { delete_series } from "@/app/actions/series";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useSession } from "@/context/session_context";
import { Series } from "@/lib/types";
import { DisplayDate, cn, slugifier } from "@/lib/utils";
import { GearIcon, Share1Icon, TrashIcon } from "@radix-ui/react-icons";
import Link from "next/link";
import { HTMLAttributes } from "react";

interface SeriesCardProps extends HTMLAttributes<HTMLDivElement> {
  series: Series;
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const SeriesCard = ({
  series,
  className,
  editable = false,
  deletable = false,
  ...props
}: SeriesCardProps) => {
  const { status } = useSession();
  return (
    <Card className={cn("grid grid-cols-2 overflow-clip", className)}>
      <div>
        <Link href={`/series/${slugifier(series.label)}-${series.id}`}>
          <CardHeader>
            <CardTitle>{series.label}</CardTitle>
          </CardHeader>
          <CardContent>
            <small>{DisplayDate(series.created_at)}</small>
          </CardContent>
        </Link>
        <CardFooter className="gap-1">
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
            <span className="sr-only">Share</span>
          </Button>
          {editable && status == "authenticated" && (
            <Button variant={"ghost"} size={"icon"} asChild>
              <Link
                href={`/series/${slugifier(series.label)}-${series.id}/edit`}
              >
                <GearIcon />
              </Link>
            </Button>
          )}
          {deletable && status == "authenticated" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button variant={"ghost"} size={"icon"}>
                  <TrashIcon />
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Are you sure?</DialogTitle>
                  <DialogDescription>Deleting {series.label}</DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose asChild>
                    <Button>Close</Button>
                  </DialogClose>
                  <DialogClose asChild>
                    <Button
                      variant={"destructive"}
                      onClick={() => {
                        delete_series(series.id).then(() => {
                          if (props.onDelete) props.onDelete(series.id);
                        });
                      }}
                    >
                      Delete
                    </Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          )}
        </CardFooter>
      </div>
      <div
        style={{
          backgroundImage: series.image
            ? `url(http://localhost:5000/api/assets/${series.image})`
            : `url(/placeholder.svg)`,
        }}
        className={cn(series.image ? "bg-contain" : "bg-center")}
      >
        <div className="w-full h-full flex items-center justify-center bg-black/50">
          <h1 className="text-card-foreground">{series.article_count}</h1>
        </div>
      </div>
    </Card>
  );
};

// <Image alt="series-image"
//   placeholder="empty"
//   fill
//   priority
//   className="object-cover"
//   src={`${series.image ?? "/placeholder.svg"}`}
// />
// <h1 className="absolute inset-0 bg-black/50 flex items-center justify-center text-muted-foreground">
//   {series.article_count}
// </h1>
export default SeriesCard;
