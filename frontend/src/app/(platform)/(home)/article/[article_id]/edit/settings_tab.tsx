"use client";

import { add_author, publish, update_article } from "@/app/actions/article";
import { get_users } from "@/app/actions/user";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Textarea } from "@/components/ui/textarea";
import { useSession } from "@/context/session_context";
import { UpdateArticleSchema } from "@/lib/definitions";
import { FullArticle, FullUser } from "@/lib/types";
import { slugifier } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import debounce from "lodash.debounce";
import { Search, XIcon } from "lucide-react";
import { redirect } from "next/navigation";
import {
  HTMLAttributes,
  useEffect,
  useMemo,
  useState,
  useTransition,
} from "react";
import { useForm } from "react-hook-form";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { z } from "zod";

interface ArticleSettingsTabProps extends HTMLAttributes<HTMLDivElement> {
  article: FullArticle;
}

const ArticleSettingsTab = ({ article }: ArticleSettingsTabProps) => {
  const { data: session } = useSession();

  const [pending, startTransition] = useTransition();

  const UpdateArticleForm = useForm<z.infer<typeof UpdateArticleSchema>>({
    resolver: zodResolver(UpdateArticleSchema),
    defaultValues: {
      title: article.title,
      description: article.description,
    },
  });

  const onSubmit = async (values: z.infer<typeof UpdateArticleSchema>) => {
    startTransition(async () => {
      const res = await update_article(article.id, values);
      if (res) redirect(`/article/${slugifier(res.title)}-${res.id}/edit`);
    });
  };

  const [query, setQuery] = useState("");

  const [users, setUsers] = useState<FullUser[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_users({
        query: query,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setUsers([
          ...users,
          ...data.items.filter((u) => u.id != session?.user_id),
        ]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasMore(false);
        }
      });
      setIsLoading(false);
    },
  });

  const handleSearch = (value: string) => {
    setQuery(value);
    setUsers([]);
    setHasMore(true);
  };

  const debouncedResults = useMemo(() => {
    return debounce(handleSearch, 300);
  }, []);

  useEffect(() => {
    return () => {
      debouncedResults.cancel();
    };
  });

  return (
    <div className="max-w-lg mx-auto">
      <div className="space-y-6">
        <div>
          <h3 className="text-lg font-medium">Article</h3>
          <p className="text-sm text-muted-foreground">
            Update your article settings. Set your preferred language and tags.
          </p>
        </div>
        <Separator />
        <Form {...UpdateArticleForm}>
          <form
            onSubmit={UpdateArticleForm.handleSubmit(onSubmit)}
            className="space-y-8"
          >
            <FormField
              control={UpdateArticleForm.control}
              name="title"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Title</FormLabel>
                  <FormControl>
                    <Input placeholder="Article Title" {...field} />
                  </FormControl>
                  <FormDescription>
                    This is the title that will be displayed on this article.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={UpdateArticleForm.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="Article description. (Optional)"
                      className="resize-none h-40"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex flex-row justify-between">
              <Button disabled={pending} type="submit">
                Update Article
              </Button>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant={"ghost"}>Publish</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Are you sure?</DialogTitle>
                    <DialogDescription>
                      Publish {article.title}
                    </DialogDescription>
                  </DialogHeader>
                  <DialogFooter>
                    <DialogClose asChild>
                      <Button>Close</Button>
                    </DialogClose>
                    <DialogClose asChild>
                      <Button
                        variant={"destructive"}
                        onClick={() => {
                          publish(article.id);
                        }}
                      >
                        Publish
                      </Button>
                    </DialogClose>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </div>
          </form>
        </Form>
        <div className="h-[500px] flex flex-col">
          <div className="sticky w-full p-4 space-y-4">
            <div className="relative">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                type="text"
                onChange={(e) => debouncedResults(e.target.value)}
                placeholder="Search by username"
                className="px-8"
              />
              <XIcon className="absolute right-2 top-2.5 h-4 w-4 text-muted-foreground" />
            </div>
          </div>
          <Separator orientation="horizontal" />
          <div className="p-4 space-y-4">
            {users.map((user) => (
              <div
                key={user.id}
                className="flex flex-row items-center justify-between whitespace-nowrap rounded-md text-sm font-medium bg-accent px-4 py-1"
              >
                <Avatar>
                  <AvatarImage
                    src={
                      user.image
                        ? "http://localhost:5000/api/assets/" + user.image
                        : undefined
                    }
                    className="object-cover"
                    alt="@avatar"
                  />
                  <AvatarFallback>{user.username.at(0)}</AvatarFallback>
                </Avatar>
                <div className="ml-2 grow spacy-y-4">
                  <h4>{user.username}</h4>
                </div>
                <Button
                  size={"sm"}
                  onClick={() => {
                    add_author(article.id, user.id).then((res) => {
                      setUsers(users.filter((u) => u.id != user.id));
                    });
                  }}
                >
                  Add Collaborator
                </Button>
              </div>
            ))}
            {(isLoading || hasMore) && (
              <div className="w-full h-20" ref={ref} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ArticleSettingsTab;
