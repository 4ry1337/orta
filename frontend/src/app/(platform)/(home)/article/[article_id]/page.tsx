"use client";

import { get_id } from "@/lib/utils";
import { Skeleton } from "@/components/ui/skeleton";
import useSWR from "swr";
import { get_article } from "@/app/actions/article";
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

interface IParams {
  article_id: string;
}

const ArticlePage = ({ params }: { params: IParams }) => {
  const { data: article } = useSWR(get_id(params.article_id), get_article);
  if (article) {
    return (
      <div>
        <div className="pt-6 px-4 space-y-4">
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
          <div>
            <UserList users={article.users || []} />
          </div>
        </div>
        <Separator />
        <Preview article={article} />
      </div>
    );
  }

  return <Skeleton className="w-full min-h-screen" />;
};

export default ArticlePage;
