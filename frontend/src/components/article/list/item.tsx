import { DisplayDate, cn } from "@/lib/utils";
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

interface ArticleCardProps extends HTMLAttributes<HTMLDivElement> {
  article: FullArticle;
}

const ArticleCard = ({ article }: ArticleCardProps) => {
  return (
    <Card>
      <Link href={`/article/${article.slug}`}>
        <CardHeader>
          <CardTitle>{article.title}</CardTitle>
        </CardHeader>
        <CardContent>
          <div>
            2 years ago when I was still working as a Graphic Designer, there
            was a UI/ UX Designer who told me “Hey, you should move to the UI/
            UX...
          </div>
          <small>{DisplayDate(article.created_at)}</small>
        </CardContent>
      </Link>
      <CardFooter className="justify-between">
        <div className="flex items-center justify-between">
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
