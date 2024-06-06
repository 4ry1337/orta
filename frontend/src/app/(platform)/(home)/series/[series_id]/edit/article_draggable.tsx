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
import { useDraggable } from "@dnd-kit/core";

interface ArticleCardProps extends HTMLAttributes<HTMLDivElement> {
  article: FullArticle;
}

const ArticleDraggableCard = ({ article, ...props }: ArticleCardProps) => {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: article.id,
    data: article,
  });

  const style = transform
    ? {
      transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
    }
    : undefined;

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
                      <AvatarImage
                        src={"http://localhost:5000/api/assets/" + user.image}
                        className="object-cover"
                        alt="@avatar"
                      />
                      <AvatarFallback>{user.username.charAt(0)}</AvatarFallback>
                    </Avatar>
                    <div className="text-primary underline-offset-4 hover:underline">
                      {user.username}
                    </div>
                  </div>
                );
              })}
            <ScrollBar orientation="horizontal" />
          </ScrollArea>
        </div>
      </CardFooter>
    </Card>
  );
};

export default ArticleDraggableCard;
