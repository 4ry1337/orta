"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSession } from "@/context/session_context";
import { Skeleton } from "@/components/ui/skeleton";
import Link from "next/link";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { z } from "zod";
import { useState, useTransition } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { CreateArticleSchema, CreateSeriesSchema } from "@/lib/definitions";
import {
  create_article,
  get_articles,
  get_user_articles,
} from "@/app/actions/article";
import { Pencil1Icon } from "@radix-ui/react-icons";
import { Button } from "@/components/ui/button";
import ArticleList from "@/components/article/list/list";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { FullArticle, Series } from "@/lib/types";
import SeriesList from "@/components/series/series/list";
import { create_series, get_serieses } from "@/app/actions/series";
import { PlusIcon } from "lucide-react";

const WritePage = () => {
  const { status, data: session } = useSession({
    authenticated: true,
  });
  const [articles, setArticles] = useState<FullArticle[]>([]);
  const [serieses, setSerieses] = useState<Series[]>([]);
  const [limit, setLimit] = useState(5);
  const [artcileCursor, setArticleCursor] = useState<string | undefined>(
    undefined,
  );
  const [seriesCursor, setSeriesCursor] = useState<string | undefined>(
    undefined,
  );
  const [hasNextArticlePage, setHasNextArticlePage] = useState(true);
  const [hasNextSeriesesPage, setHasNextSeriesesPage] = useState(true);

  const [isArticleLoading, setIsArticleLoading] = useState(false);
  const [isSeriesesLoading, setIsSeriesesLoading] = useState(false);

  const [articleref] = useInfiniteScroll({
    loading: isArticleLoading,
    hasNextPage: hasNextArticlePage,
    onLoadMore: () => {
      setIsArticleLoading(true);
      get_articles({
        username: session!.username,
        cursor: {
          cursor: artcileCursor,
          limit,
        },
      }).then((data) => {
        setArticles([...articles, ...data.items]);
        if (data.next_cursor !== null) {
          setArticleCursor(data.next_cursor);
        } else {
          setHasNextArticlePage(false);
        }
      });
      setIsArticleLoading(false);
    },
  });

  const [seriesref] = useInfiniteScroll({
    loading: isSeriesesLoading,
    hasNextPage: hasNextSeriesesPage,
    onLoadMore: () => {
      setIsSeriesesLoading(true);
      get_serieses({
        user_id: session!.user_id,
        cursor: {
          cursor: seriesCursor,
          limit,
        },
      }).then((data) => {
        setSerieses([...serieses, ...data.items]);
        if (data.next_cursor !== null) {
          setSeriesCursor(data.next_cursor);
        } else {
          setHasNextSeriesesPage(false);
        }
      });
      setIsSeriesesLoading(false);
    },
  });

  const [articlePending, startArticleTransition] = useTransition();

  const CreateArticleForm = useForm<z.infer<typeof CreateArticleSchema>>({
    resolver: zodResolver(CreateArticleSchema),
    defaultValues: {
      title: "My Story",
    },
  });

  const onArticleSubmit = async (
    values: z.infer<typeof CreateArticleSchema>,
  ) => {
    startArticleTransition(async () => {
      const res = await create_article(values);
      if (res) setArticles([res, ...articles]);
    });
  };

  const [seriesPending, startSeriesTransition] = useTransition();

  const CreateSeriesForm = useForm<z.infer<typeof CreateSeriesSchema>>({
    resolver: zodResolver(CreateSeriesSchema),
    defaultValues: {
      label: "My Series",
    },
  });

  const onSeriesSubmit = async (values: z.infer<typeof CreateSeriesSchema>) => {
    startSeriesTransition(async () => {
      const res = await create_series(values);
      if (res) setSerieses([res, ...serieses]);
    });
  };
  if (status == "loading") {
    return <Skeleton className="h-screen" />;
  }

  return (
    <Tabs defaultValue="article" className="p-4">
      <TabsList>
        <TabsTrigger value="article">
          <Link href={"#article"}>Article</Link>
        </TabsTrigger>
        <TabsTrigger value="series">
          <Link href={"#series"}>Series</Link>
        </TabsTrigger>
      </TabsList>
      <TabsContent value="article" className="space-y-4">
        <Dialog>
          <DialogTrigger asChild>
            <Button
              variant={"ghost"}
              className="w-full h-auto rounded-xl p-6 border text-card-foreground shadow border-dashed"
            >
              <h3 className="flex gap-2 justify-center items-center font-semibold leading-none tracking-tight text-muted-foreground">
                <Pencil1Icon className="h-7 w-7" />
                Create Article
              </h3>
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create Article</DialogTitle>
            </DialogHeader>
            <Form {...CreateArticleForm}>
              <form
                id="create_article"
                className="grid py-4"
                onSubmit={CreateArticleForm.handleSubmit(onArticleSubmit)}
              >
                <FormField
                  control={CreateArticleForm.control}
                  name="title"
                  render={({ field }) => (
                    <FormItem className="grid grid-cols-4 items-center gap-4">
                      <FormLabel>Title</FormLabel>
                      <FormControl>
                        <Input {...field} className="col-span-3" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </form>
            </Form>
            <DialogFooter>
              <DialogClose asChild>
                <Button type="button" variant="secondary">
                  Cancel
                </Button>
              </DialogClose>
              <DialogClose asChild>
                <Button
                  disabled={articlePending}
                  form="create_article"
                  type="submit"
                >
                  Create Article
                </Button>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <ArticleList
          editable
          deletable
          articles={articles}
          onDelete={(id) => {
            setArticles(articles.filter((article) => article.id !== id));
          }}
        />
        {(isArticleLoading || hasNextArticlePage) && (
          <div className="w-full h-20" ref={articleref} />
        )}
      </TabsContent>
      <TabsContent value="series" className="space-y-4">
        <Dialog>
          <DialogTrigger asChild>
            <Button
              disabled={seriesPending}
              variant={"ghost"}
              className="w-full h-auto rounded-xl p-6 border text-card-foreground shadow border-dashed"
            >
              <h3 className="flex gap-2 justify-center items-center font-semibold leading-none tracking-tight text-muted-foreground">
                <PlusIcon className="h-7 w-7" />
                Create Series
              </h3>
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Create Series</DialogTitle>
            </DialogHeader>
            <Form {...CreateSeriesForm}>
              <form
                id="create_series"
                className="grid grid-2 py-4"
                onSubmit={CreateSeriesForm.handleSubmit(onSeriesSubmit)}
              >
                <FormField
                  control={CreateSeriesForm.control}
                  name="label"
                  render={({ field }) => (
                    <FormItem className="grid grid-cols-4 items-center gap-4">
                      <FormLabel>Label</FormLabel>
                      <FormControl>
                        <Input {...field} className="col-span-3" />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </form>
            </Form>
            <DialogFooter>
              <DialogClose asChild>
                <Button type="button" variant="secondary">
                  Cancel
                </Button>
              </DialogClose>
              <DialogClose asChild>
                <Button form="create_series" type="submit">
                  Create
                </Button>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <div className="grid grid-cols-2 lg:grid-cols-3 gap-4">
          <SeriesList
            editable
            deletable
            onDelete={(id) => {
              setSerieses(serieses.filter((series) => series.id !== id));
            }}
            serieses={serieses}
          />
          {(isSeriesesLoading || hasNextSeriesesPage) && (
            <div className="w-full h-20" ref={seriesref} />
          )}
        </div>
      </TabsContent>
    </Tabs>
  );
};

export default WritePage;
