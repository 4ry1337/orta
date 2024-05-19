"use client";

import { get_articles } from "@/app/actions/article";
import CreateArticleDialog from "@/components/article/create_article_dialog";
import ArticleList from "@/components/article/list/list";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { usePathname } from "next/navigation";
import { useRouter } from "next/navigation";
import useSWR from "swr";

const WritePage = ({
  searchParams,
}: {
  searchParams: {
    page: number | null | undefined;
    per_page: number | null | undefined;
  };
}) => {
  const router = useRouter();
  const pathname = usePathname();

  const { status, data } = useSession({
    authenticated: true,
  });
  const { data: articles, error } = useSWR(
    status == "loading"
      ? null
      : {
        usernames: [data.username],
        pagination: {
          page: searchParams.page,
          per_page: searchParams.per_page,
        },
      },
    get_articles,
    {
      onSuccess(data, key, config) {
        const params = new URLSearchParams();
        params.set("page", data.pagination.page.toString());
        params.set("per_page", data.pagination.per_page.toString());
        router.push(pathname + "?" + params.toString());
      },
    },
  );

  if (status == "loading") {
    return <Skeleton className="h-screen w-full" />;
  }

  return (
    <div className="">
      <div className="flex p-4">
        <h1 className="text-lg font-medium grow">Your Articles</h1>
        <CreateArticleDialog />
      </div>
      <Separator />
      <div>
        {!!error && <div className="">{error.message}</div>}
        {!!articles && <ArticleList articles={articles.items} />}
      </div>
    </div>
  );
};

export default WritePage;
