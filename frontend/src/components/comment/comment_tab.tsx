import {
  create_article_comment,
  get_article_comments,
} from "@/app/actions/article";
import { Comment, FullComment } from "@/lib/types";
import { HTMLAttributes, useState, useTransition } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { Textarea } from "../ui/textarea";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "../ui/form";
import { useForm } from "react-hook-form";
import { CreateCommentSchema } from "@/lib/definitions";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { Button } from "../ui/button";
import CommentList from "./list/list";
import { AutosizeTextarea } from "../ui/autoresize_textarea";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { useSession } from "@/context/session_context";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "../ui/card";
import { create_list_comment, get_list_comments } from "@/app/actions/list";
import { ScrollArea, ScrollBar } from "../ui/scroll-area";
import {
  create_series_comment,
  get_series_comments,
} from "@/app/actions/series";

// interface CommentsProps extends HTMLAttributes<HTMLDivElement> {
//   article_id: string;
// }

export const ArticleComments = ({ article_id }: { article_id: string }) => {
  const { status, data: session } = useSession();

  const [pending, startTransition] = useTransition();

  const CreateCommentForm = useForm<z.infer<typeof CreateCommentSchema>>({
    resolver: zodResolver(CreateCommentSchema),
    defaultValues: {
      content: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateCommentSchema>) => {
    startTransition(async () => {
      const res = await create_article_comment(article_id, values);
      if (res) {
        const comment: FullComment = {
          ...res,
          username: session!.username,
          image: session?.image,
          followed: false,
        };
        setComments([comment, ...comments]);
      }
    });
  };

  const [query, setQuery] = useState("");

  const [comments, setComments] = useState<FullComment[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasNextPage, setHasNextPage] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      get_article_comments(article_id, query, {
        cursor,
        limit,
      }).then((data) => {
        setComments([...comments, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  return (
    <>
      <h4>Comments:</h4>
      {status == "authenticated" && (
        <Form {...CreateCommentForm}>
          <form onSubmit={CreateCommentForm.handleSubmit(onSubmit)}>
            <Card>
              <CardHeader>
                <CardTitle>
                  <div className="inline-flex items-center gap-2 whitespace-nowrap rounded-md text-sm font-medium">
                    <Avatar>
                      <AvatarImage
                        src={
                          session.image &&
                          "http://localhost:5000/api/assets/" + session.image
                        }
                        className="object-cover"
                        alt="@avatar"
                      />
                      <AvatarFallback>{session.username.at(0)}</AvatarFallback>
                    </Avatar>
                    <span className="ml-2 hidden xl:block">
                      {session.username}
                    </span>
                  </div>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <FormField
                  control={CreateCommentForm.control}
                  name="content"
                  render={({ field }) => (
                    <FormItem>
                      <FormControl>
                        <AutosizeTextarea
                          placeholder="Write your feedback."
                          // className="prose prose-neutral prose-sm dark:prose-invert"
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </CardContent>
              <CardFooter className="justify-end">
                <Button className="" disabled={pending} type="submit">
                  Add Comment
                </Button>
              </CardFooter>
            </Card>
          </form>
        </Form>
      )}
      <ScrollArea className="mt-4">
        <div className="space-y-4">
          <CommentList comments={comments} />
          {(isLoading || hasNextPage) && (
            <div className="w-full h-20" ref={ref} />
          )}
        </div>
        <ScrollBar orientation="vertical" />
      </ScrollArea>
    </>
  );
};

export const ListComments = ({ list_id }: { list_id: string }) => {
  const { status, data: session } = useSession();

  const [pending, startTransition] = useTransition();

  const CreateCommentForm = useForm<z.infer<typeof CreateCommentSchema>>({
    resolver: zodResolver(CreateCommentSchema),
    defaultValues: {
      content: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateCommentSchema>) => {
    startTransition(async () => {
      const res = await create_list_comment(list_id, values);
      if (res) {
        const comment: FullComment = {
          ...res,
          username: session!.username,
          image: session?.image,
          followed: false,
        };
        setComments([comment, ...comments]);
      }
    });
  };

  const [query, setQuery] = useState("");

  const [comments, setComments] = useState<FullComment[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasNextPage, setHasNextPage] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      get_list_comments(list_id, query, {
        cursor,
        limit,
      }).then((data) => {
        setComments([...comments, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  return (
    <>
      <h4>Comments:</h4>
      {status == "authenticated" && (
        <Form {...CreateCommentForm}>
          <form onSubmit={CreateCommentForm.handleSubmit(onSubmit)}>
            <Card>
              <CardHeader>
                <CardTitle>
                  <div className="inline-flex items-center gap-2 whitespace-nowrap rounded-md text-sm font-medium">
                    <Avatar>
                      <AvatarImage
                        src={
                          session.image &&
                          "http://localhost:5000/api/assets/" + session.image
                        }
                        className="object-cover"
                        alt="@avatar"
                      />
                      <AvatarFallback>{session.username.at(0)}</AvatarFallback>
                    </Avatar>
                    <span className="ml-2 hidden xl:block">
                      {session.username}
                    </span>
                  </div>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <FormField
                  control={CreateCommentForm.control}
                  name="content"
                  render={({ field }) => (
                    <FormItem>
                      <FormControl>
                        <AutosizeTextarea
                          placeholder="Write your feedback."
                          // className="prose prose-neutral prose-sm dark:prose-invert"
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </CardContent>
              <CardFooter className="justify-end">
                <Button className="" disabled={pending} type="submit">
                  Add Comment
                </Button>
              </CardFooter>
            </Card>
          </form>
        </Form>
      )}
      <ScrollArea>
        <div className="space-y-4">
          <CommentList comments={comments} />
          {(isLoading || hasNextPage) && (
            <div className="w-full h-20" ref={ref} />
          )}
        </div>
        <ScrollBar orientation="vertical" />
      </ScrollArea>
    </>
  );
};

export const SeriesComments = ({ series_id }: { series_id: string }) => {
  const { status, data: session } = useSession();

  const [pending, startTransition] = useTransition();

  const CreateCommentForm = useForm<z.infer<typeof CreateCommentSchema>>({
    resolver: zodResolver(CreateCommentSchema),
    defaultValues: {
      content: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateCommentSchema>) => {
    startTransition(async () => {
      const res = await create_series_comment(series_id, values);
      if (res) {
        const comment: FullComment = {
          ...res,
          username: session!.username,
          image: session?.image,
          followed: false,
        };
        setComments([comment, ...comments]);
      }
    });
  };

  const [query, setQuery] = useState("");

  const [comments, setComments] = useState<FullComment[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasNextPage, setHasNextPage] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      get_series_comments(series_id, query, {
        cursor,
        limit,
      }).then((data) => {
        setComments([...comments, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasNextPage(false);
        }
      });
      setIsLoading(false);
    },
  });

  return (
    <>
      <h4>Comments:</h4>
      {status == "authenticated" && (
        <Form {...CreateCommentForm}>
          <form onSubmit={CreateCommentForm.handleSubmit(onSubmit)}>
            <Card>
              <CardHeader>
                <CardTitle>
                  <div className="inline-flex items-center gap-2 whitespace-nowrap rounded-md text-sm font-medium">
                    <Avatar>
                      <AvatarImage
                        src={
                          session.image &&
                          "http://localhost:5000/api/assets/" + session.image
                        }
                        className="object-cover"
                        alt="@avatar"
                      />
                      <AvatarFallback>{session.username.at(0)}</AvatarFallback>
                    </Avatar>
                    <span className="ml-2 hidden xl:block">
                      {session.username}
                    </span>
                  </div>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <FormField
                  control={CreateCommentForm.control}
                  name="content"
                  render={({ field }) => (
                    <FormItem>
                      <FormControl>
                        <AutosizeTextarea
                          placeholder="Write your feedback."
                          // className="prose prose-neutral prose-sm dark:prose-invert"
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </CardContent>
              <CardFooter className="justify-end">
                <Button className="" disabled={pending} type="submit">
                  Add Comment
                </Button>
              </CardFooter>
            </Card>
          </form>
        </Form>
      )}
      <ScrollArea className="mt-4">
        <div className="space-y-4">
          <CommentList comments={comments} />
          {(isLoading || hasNextPage) && (
            <div className="w-full h-20" ref={ref} />
          )}
        </div>
        <ScrollBar orientation="vertical" />
      </ScrollArea>
    </>
  );
};
