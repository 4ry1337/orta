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

interface ArticleCardProps extends HTMLAttributes<HTMLDivElement> {
  article: FullArticle;
}

const ArticleCard = ({ article }: ArticleCardProps) => {
  return (
    <Card className="">
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
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
      </CardFooter>
    </Card>
  );
};

export default ArticleCard;
