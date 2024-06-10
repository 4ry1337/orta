import { DisplayDate } from "@/lib/utils";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { FullArticle } from "@/lib/types";
import { HTMLAttributes } from "react";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Button } from "@/components/ui/button";
import { remove_article_series } from "@/app/actions/series";

interface ArticleCardProps extends HTMLAttributes<HTMLDivElement> {
  series_id: string;
  article: FullArticle;
  onDelete: (article_id: string) => void;
}

const ArticleSortableCard = ({
  article,
  series_id,
  onDelete,
  ...props
}: ArticleCardProps) => {
  const { setNodeRef, listeners, attributes, transform, transition } =
    useSortable({
      id: article.id,
      data: article,
    });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  return (
    <Card ref={setNodeRef} style={style} {...attributes} {...listeners}>
      <CardHeader>
        <CardTitle>{article.title}</CardTitle>
      </CardHeader>
      <CardContent>
        <div>{article.description}</div>
        {article.published_at && (
          <small>{DisplayDate(article.published_at)}</small>
        )}
      </CardContent>
      <CardFooter className="justify-between ">
        <ScrollArea>
          <div className="flex flex-row items-center gap-4 justify-center">
            {article.users &&
              article.users.map((user) => {
                return (
                  <div
                    key={user.id}
                    className="inline-flex items-center justify-center"
                  >
                    <Avatar className="mr-2 w-7 h-7">
                      <AvatarImage
                        src={
                          user.image &&
                          "http://localhost:5000/api/assets/" + user.image
                        }
                        className="object-cover"
                        alt="@avatar"
                      />
                      <AvatarFallback>{user.username.charAt(0)}</AvatarFallback>
                    </Avatar>
                    <div>{user.username}</div>
                  </div>
                );
              })}
          </div>
          <ScrollBar orientation="horizontal" />
        </ScrollArea>
        <Button
          onClick={() => {
            remove_article_series(series_id, article.id).then(() => {
              onDelete(article.id);
            });
          }}
        >
          Remove
        </Button>
      </CardFooter>
    </Card>
  );
};

export default ArticleSortableCard;
