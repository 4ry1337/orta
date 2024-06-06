"use client";

import { get_id, slugifier } from "@/lib/utils";
import { Skeleton } from "@/components/ui/skeleton";
import useSWR from "swr";
import {
  get_article,
  like_article,
  unlike_article,
} from "@/app/actions/article";
import { Separator } from "@/components/ui/separator";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import UserList from "@/components/user/list/list";
import Preview from "@/components/article/preview";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import {
  BookmarkFilledIcon,
  BookmarkIcon,
  ChatBubbleIcon,
  HeartFilledIcon,
  HeartIcon,
} from "@radix-ui/react-icons";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import ListPopover from "@/components/list/list_popover";
import { FullArticle, List } from "@/lib/types";
import { useSession } from "@/context/session_context";
import Link from "next/link";

interface IParams {
  article_id: string;
}

const ArticlePage = ({ params }: { params: IParams }) => {
  const { data } = useSession();

  const { isLoading } = useSWR(get_id(params.article_id), get_article, {
    onSuccess(data) {
      if (data) {
        setArticle(data);
      }
    },
  });

  const [article, setArticle] = useState<FullArticle | null>(null);

  if (isLoading || !article) {
    return <Skeleton className="w-full min-h-screen" />;
  }

  return (
    <div>
      <div className="pt-6 p-4 space-y-4">
        <div className="p-2">
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem>
                <BreadcrumbLink href="/">Home</BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbPage>{article.title}</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </div>
        <h1 className="scroll-m-20 text-4xl text-center font-extrabold tracking-tight lg:text-5xl">
          {article.title}
        </h1>
        {article.series.length != 0 && (
          <div className="flex justify-center">
            <Button asChild variant={"link"}>
              <Link
                href={`/series/${slugifier(article.series[0].label)}-${article.series[0].id}`}
              >
                <h3>from {article.series[0].label}</h3>
              </Link>
            </Button>
          </div>
        )}
        <div className="">
          <ScrollArea>
            <div className="flex flex-row gap-4">
              <UserList badge users={article.users || []} />
            </div>
            <ScrollBar orientation="horizontal" />
          </ScrollArea>
        </div>
        <div className="flex flex-row  items-center justify-between text-muted-foreground">
          <div className="inline-flex gap-4">
            <div className="inline-flex gap-2 items-center">
              {article.liked ? (
                <Button
                  disabled={!data}
                  onClick={() => {
                    unlike_article(article.id);
                    setArticle({
                      ...article,
                      liked: false,
                      like_count: article.like_count - 1,
                    });
                  }}
                  variant={"ghost"}
                  size={"icon"}
                >
                  <HeartFilledIcon className="w-6 h-6" />
                </Button>
              ) : (
                <Button
                  disabled={!data}
                  onClick={() => {
                    like_article(article.id);
                    setArticle({
                      ...article,
                      liked: true,
                      like_count: article.like_count + 1,
                    });
                  }}
                  variant={"ghost"}
                  size={"icon"}
                >
                  <HeartIcon className="w-6 h-6" />
                </Button>
              )}
              <span className="w-4">{article.like_count}</span>
            </div>
            <div className="inline-flex gap-2 items-center">
              <ChatBubbleIcon className="w-6 h-6" />
              <span className="w-4">{article.comment_count}</span>
            </div>
          </div>
          <div className="inline-flex gap-4">
            {data && <ListPopover article={article} />}
          </div>
        </div>
      </div>
      <Separator />
      <div className="p-4">
        {article.content ? (
          <Preview content={article.content} />
        ) : (
          <div className="h-96 content-center">
            <h1 className="text-center">No content</h1>
          </div>
        )}
      </div>
    </div>
  );
};

export default ArticlePage;
