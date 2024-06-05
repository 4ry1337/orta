import { DisplayDate, slugifier } from "@/lib/utils";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Share1Icon } from "@radix-ui/react-icons";
import Link from "next/link";
import { FullArticle } from "@/lib/types";
import { HTMLAttributes } from "react";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { useSession } from "@/context/session_context";
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
import { delete_article } from "@/app/actions/article";
import ListPopover from "@/components/list/list_popover";

interface ArticleCardProps extends HTMLAttributes<HTMLDivElement> {
  article: FullArticle;
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const ArticleCard = ({
  article,
  editable = false,
  deletable = false,
  ...props
}: ArticleCardProps) => {
  const { status } = useSession();
  return (
    <Card {...props}>
      <Link href={`/article/${slugifier(article.title)}-${article.id}`}>
        <CardHeader>
          <CardTitle>{article.title}</CardTitle>
        </CardHeader>
        <CardContent>
          <div>{article.description}</div>
          <small>{DisplayDate(article.created_at)}</small>
        </CardContent>
      </Link>
      <CardFooter className="justify-between ">
        <div className="flex items-center justify-between">
          <ScrollArea>
            {article.users &&
              article.users.map((user) => {
                return (
                  <div
                    key={user.id}
                    className="inline-flex items-center justify-center"
                  >
                    <Avatar className="mr-2 w-7 h-7">
                      <AvatarImage src={user.image} alt="@avatar" />
                      <AvatarFallback>{user.username.charAt(0)}</AvatarFallback>
                    </Avatar>
                    <Link
                      className="text-primary underline-offset-4 hover:underline"
                      href={`/${user.username}`}
                    >
                      {user.username}
                    </Link>
                  </div>
                );
              })}
            <ScrollBar orientation="horizontal" />
          </ScrollArea>
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
                  <DialogDescription>
                    Deleting {article.title}
                  </DialogDescription>
                </DialogHeader>
                <DialogFooter>
                  <DialogClose asChild>
                    <Button>Close</Button>
                  </DialogClose>
                  <DialogClose asChild>
                    <Button
                      variant={"destructive"}
                      onClick={() => {
                        delete_article(article.id).then(() => {
                          if (props.onDelete) props.onDelete(article.id);
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
            <Button variant={"ghost"} asChild>
              <Link
                href={`/article/${slugifier(article.title)}-${article.id}/edit`}
              >
                Edit
              </Link>
            </Button>
          )}
          {status == "authenticated" && <ListPopover article={article} />}
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
      </CardFooter>
    </Card>
  );
};

export default ArticleCard;
