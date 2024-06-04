import { delete_list } from "@/app/actions/list";
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
import { List } from "@/lib/types";
import { DisplayDate, slugifier } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import Link from "next/link";
import { HTMLAttributes } from "react";

interface ListCardProps extends HTMLAttributes<HTMLDivElement> {
  list: List;
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const ListCard = ({
  list,
  deletable,
  editable,
  onDelete,
  ...props
}: ListCardProps) => {
  const { status } = useSession();
  return (
    <Card {...props}>
      <Link href={`/list/${slugifier(list.label)}-${list.id}`}>
        <CardHeader>
          <CardTitle>{list.label}</CardTitle>
        </CardHeader>
        <CardContent>
          <small>{DisplayDate(list.created_at)}</small>
        </CardContent>
      </Link>
      <CardFooter className="justify-between">
        <div className="">
          <small className="text-muted-foreground">
            {list.article_count} Articles
          </small>
        </div>
        <div className="inline-flex gap-2">
          {deletable && status == "authenticated" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button variant={"ghost"}>Delete</Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Are you sure?</DialogTitle>
                  <DialogDescription>Deleting {list.label}</DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose asChild>
                    <Button>Close</Button>
                  </DialogClose>
                  <DialogClose asChild>
                    <Button
                      variant={"destructive"}
                      onClick={() => {
                        delete_list(list.id).then(() => {
                          if (onDelete) onDelete(list.id);
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
          {editable && status == "authenticated" && (
            <Button variant={"ghost"}>Edit</Button>
          )}
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
      </CardFooter>
    </Card>
  );
};

export default ListCard;
