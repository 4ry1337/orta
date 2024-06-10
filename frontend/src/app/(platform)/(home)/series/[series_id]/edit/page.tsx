"use client";

import { get_articles } from "@/app/actions/article";
import { get_user_articles } from "@/app/actions/user";
import {
  add_article_series,
  get_series,
  get_series_articles,
  remove_article_series,
  reorder_article_series,
} from "@/app/actions/series";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { FullArticle, Series } from "@/lib/types";
import { DisplayDate, get_id } from "@/lib/utils";
import { useState, useTransition } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import useSWR from "swr";
import {
  DndContext,
  DragEndEvent,
  MouseSensor,
  TouchSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import { SortableContext, arrayMove } from "@dnd-kit/sortable";
import Aside from "@/components/aside";
import { Button } from "@/components/ui/button";
import { Share1Icon } from "@radix-ui/react-icons";
import { update_series } from "@/app/actions/series";
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
import Image from "next/image";
import { PlusIcon } from "lucide-react";
import { upload_asset } from "@/app/actions/asset";
import {
  FileInput,
  FileUploader,
  FileUploaderContent,
  FileUploaderItem,
} from "@/components/ui/file_upload";
import { DropzoneOptions } from "react-dropzone";
import { AspectRatio } from "@/components/ui/aspect-ratio";
import { Input } from "@/components/ui/input";
import { UpdateSeriesSchema, UploadAssetFormSchema } from "@/lib/definitions";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import ArticleSortableCard from "./article_sortable";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

interface IParams {
  series_id: string;
}

const SeriesEditPage = ({ params }: { params: IParams }) => {
  const { data: session, status } = useSession({
    authenticated: true,
  });

  useSWR(get_id(params.series_id), get_series, {
    onSuccess(data, key, config) {
      setSeries(data);
      UpdateSeriesForm.setValue("label", data.label);
      UpdateSeriesForm.setValue("image", data.image);
    },
  });

  const [series, setSeries] = useState<Series | undefined>(undefined);

  const [limit, setLimit] = useState(10);

  const [seriesArticles, setSeriesArticles] = useState<FullArticle[]>([]);

  const [seriesCursor, setSeriesCursor] = useState<string | undefined>(
    undefined,
  );

  const [hasNextSeriesPage, setHasNextSeriesPage] = useState(true);

  const [isSeriesLoading, setIsSeriesLoading] = useState(false);

  const [seriesRef] = useInfiniteScroll({
    loading: isSeriesLoading,
    hasNextPage: hasNextSeriesPage,
    onLoadMore: () => {
      setIsSeriesLoading(true);
      if (!session || !series) return;
      get_series_articles(series.id, {
        cursor: seriesCursor,
        limit,
      }).then((data) => {
        setSeriesArticles([...seriesArticles, ...data.items]);
        if (data.next_cursor !== null) {
          setSeriesCursor(data.next_cursor);
        } else {
          setHasNextSeriesPage(false);
        }
      });
      setIsSeriesLoading(false);
    },
  });

  const [pending, startTransition] = useTransition();

  const UploadAssetForm = useForm<z.infer<typeof UploadAssetFormSchema>>({
    resolver: zodResolver(UploadAssetFormSchema),
  });

  const dropzone = {
    accept: {
      "image/*": [".jpg", ".jpeg", ".png"],
    },
    multiple: false,
    maxFiles: 1,
    maxSize: 8 * 1024 * 1024,
  } satisfies DropzoneOptions;

  const onUploadSubmit = async (
    values: z.infer<typeof UploadAssetFormSchema>,
  ) => {
    startTransition(async () => {
      const res = await upload_asset(values);
      if (res && series) {
        setSeries({ ...series, image: res });
        await update_series(series!.id, {
          image: res,
        });
      }
    });
  };

  const UpdateSeriesForm = useForm<z.infer<typeof UpdateSeriesSchema>>({
    resolver: zodResolver(UpdateSeriesSchema),
    mode: "onChange",
  });

  const onSubmit = async (values: z.infer<typeof UpdateSeriesSchema>) => {
    startTransition(async () => {
      await update_series(series!.id, values);
    });
  };

  function ReorderArticle(event: DragEndEvent) {
    const { over, active } = event;
    if (series && over && active.id !== over.id) {
      setSeriesArticles((items) => {
        const oldIndex = items.findIndex((item) => item.id === active.id);
        const newIndex = items.findIndex((item) => item.id === over.id);
        if (newIndex === seriesArticles.length - 1) {
          reorder_article_series(
            series.id,
            seriesArticles[oldIndex].id,
            seriesArticles[newIndex].order! + 100,
          );
        } else if (newIndex === 0) {
          reorder_article_series(
            series.id,
            seriesArticles[oldIndex].id,
            seriesArticles[newIndex].order! - 100,
          );
        } else {
          reorder_article_series(
            series.id,
            seriesArticles[oldIndex].id,
            (seriesArticles[newIndex].order! +
              seriesArticles[newIndex + 1].order!) /
            2,
          );
        }

        return arrayMove(items, oldIndex, newIndex);
      });
    }
  }

  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [articlesCursor, setArticlesCursor] = useState<string | undefined>(
    undefined,
  );

  const [hasArticlesNextPage, setHasArticlesNextPage] = useState(true);

  const [isArticlesLoading, setIsArticlesLoading] = useState(false);

  const [articlesRef] = useInfiniteScroll({
    loading: isArticlesLoading,
    hasNextPage: hasArticlesNextPage,
    onLoadMore: () => {
      setIsArticlesLoading(true);
      if (!series || !session) return;
      get_user_articles(session.username, {
        cursor: articlesCursor,
        limit,
      }).then((data) => {
        console.log(data);
        setArticles([
          ...articles,
          ...data.items.filter((a) => a.series.length == 0),
        ]);
        if (data.next_cursor !== null) {
          setArticlesCursor(data.next_cursor);
        } else {
          setHasArticlesNextPage(false);
        }
      });
      setIsArticlesLoading(false);
    },
  });

  const sensors = useSensors(
    useSensor(MouseSensor, {
      // Require the mouse to move by 10 pixels before activating
      activationConstraint: {
        distance: 10,
      },
    }),
    useSensor(TouchSensor, {
      // Press delay of 250ms, with tolerance of 5px of movement
      activationConstraint: {
        delay: 250,
        tolerance: 5,
      },
    }),
  );
  if (!series || status == "loading")
    return <Skeleton className="w-full h-full" />;

  return (
    <div className="flex">
      <div className="w-full">
        <Form {...UpdateSeriesForm}>
          <form
            onSubmit={UpdateSeriesForm.handleSubmit(onSubmit)}
            className="p-4 flex flex-row items-center justify-between"
          >
            <div className="space-y-4">
              <FormField
                control={UpdateSeriesForm.control}
                name="label"
                render={({ field }) => (
                  <FormItem>
                    <FormControl>
                      <Input {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="flex gap-4">
                <Button disabled={pending} variant={"secondary"} type="submit">
                  Update series
                </Button>
                <Dialog>
                  <DialogTrigger asChild>
                    <Button>Add Article</Button>
                  </DialogTrigger>
                  <DialogContent className="max-w-xl">
                    <DialogHeader>
                      <DialogTitle>Add Article</DialogTitle>
                    </DialogHeader>
                    <ScrollArea className="max-h-96">
                      <div className="space-y-4">
                        {articles.map((article) => (
                          <Card key={article.id}>
                            <CardHeader>
                              <CardTitle>{article.title}</CardTitle>
                            </CardHeader>
                            <CardContent>
                              <div>{article.description}</div>
                              {article.published_at && (
                                <small>
                                  {DisplayDate(article.published_at)}
                                </small>
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
                                                "http://localhost:5000/api/assets/" +
                                                user.image
                                              }
                                              className="object-cover"
                                              alt="@avatar"
                                            />
                                            <AvatarFallback>
                                              {user.username.charAt(0)}
                                            </AvatarFallback>
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
                                  add_article_series(
                                    series.id,
                                    article.id,
                                  ).then(() => {
                                    setSeriesArticles([
                                      ...seriesArticles,
                                      { ...article, series: [series] },
                                    ]);
                                    setArticles(
                                      articles.filter(
                                        (e) => e.id !== article.id,
                                      ),
                                    );
                                  });
                                }}
                              >
                                Add
                              </Button>
                            </CardFooter>
                          </Card>
                        ))}
                      </div>
                      {(isArticlesLoading || hasArticlesNextPage) && (
                        <div className="w-full h-20" ref={articlesRef} />
                      )}
                      <ScrollBar />
                    </ScrollArea>
                  </DialogContent>
                </Dialog>
              </div>
            </div>
            <div className="relative rounded-md overflow-clip">
              <Image
                src={
                  series.image
                    ? "http://localhost:5000/api/assets/" + series.image
                    : "/placeholder.svg"
                }
                width={400}
                height={400}
                className="object-cover"
                alt="@avatar"
              />
              <Dialog>
                <DialogTrigger className="absolute top-0 bottom-0 left-0 right-0">
                  <div className="p-4 w-full h-full flex items-end justify-end hover:bg-black/50">
                    <PlusIcon />
                  </div>
                </DialogTrigger>
                <DialogContent className="flex flex-col gap-6">
                  <DialogHeader>
                    <DialogTitle>Upload profile image</DialogTitle>
                  </DialogHeader>
                  <Form {...UploadAssetForm}>
                    <form
                      id="image"
                      onSubmit={UploadAssetForm.handleSubmit(onUploadSubmit)}
                    >
                      <FormField
                        control={UploadAssetForm.control}
                        name="files"
                        render={({ field }) => (
                          <FormItem>
                            <FileUploader
                              value={field.value}
                              onValueChange={field.onChange}
                              dropzoneOptions={dropzone}
                              reSelect
                            >
                              {field.value && field.value.length > 0 ? (
                                <FileUploaderContent className="aspect-square">
                                  {field.value.map((file, i) => (
                                    <FileUploaderItem
                                      key={i}
                                      index={i}
                                      aria-roledescription={`file ${i + 1} containing ${file.name}`}
                                      className="w-full h-full"
                                    >
                                      <AspectRatio className="size-full">
                                        <Image
                                          src={URL.createObjectURL(file)}
                                          alt={file.name}
                                          className="object-cover rounded-md"
                                          fill
                                        />
                                      </AspectRatio>
                                    </FileUploaderItem>
                                  ))}
                                </FileUploaderContent>
                              ) : (
                                <FileInput className="aspect-square border-dashed border">
                                  <div className="w-full h-full flex justify-center items-center">
                                    <p className="text-gray-400">
                                      Drop file here
                                    </p>
                                  </div>
                                </FileInput>
                              )}
                            </FileUploader>
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
                      <Button disabled={pending} form="image" type="submit">
                        Upload
                      </Button>
                    </DialogClose>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </div>
          </form>
        </Form>
        <Separator className="w-full" orientation="horizontal" />
        <div className="p-4 space-y-4">
          <DndContext sensors={sensors} onDragEnd={ReorderArticle}>
            <SortableContext items={seriesArticles}>
              {seriesArticles.map((article) => (
                <ArticleSortableCard
                  series_id={series.id}
                  key={article.id}
                  onDelete={(article_id) => {
                    setSeriesArticles(
                      seriesArticles.filter(
                        (article) => article.id !== article_id,
                      ),
                    );
                    setArticles([...articles, { ...article, series: [] }]);
                  }}
                  article={article}
                />
              ))}
              {(isSeriesLoading || hasNextSeriesPage) && (
                <div className="w-full h-20" ref={seriesRef} />
              )}
            </SortableContext>
          </DndContext>
        </div>
      </div>
    </div>
  );
};

export default SeriesEditPage;
