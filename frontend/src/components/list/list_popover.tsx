import React, { useState, useTransition } from "react";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { Button } from "../ui/button";
import {
  BookmarkFilledIcon,
  BookmarkIcon,
  PlusIcon,
} from "@radix-ui/react-icons";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";
import { useSession } from "@/context/session_context";
import { FullArticle, List } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import {
  add_article,
  create_list,
  get_lists,
  remove_article,
} from "@/app/actions/list";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { CreateListSchema } from "@/lib/definitions";
import { zodResolver } from "@hookform/resolvers/zod";
import { Checkbox } from "../ui/checkbox";

const ListPopover = ({ article }: { article: FullArticle }) => {
  const { status, data } = useSession({
    authenticated: true,
  });

  const [bookmark, setBookmark] = useState(article.lists);
  const [pending, startTransition] = useTransition();
  const [lists, setLists] = useState<List[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [cursor, setCursor] = useState<string | undefined>(undefined);
  const [hasNextPage, setHasNextPage] = useState(true);

  const CreateListForm = useForm<z.infer<typeof CreateListSchema>>({
    resolver: zodResolver(CreateListSchema),
    defaultValues: {
      label: "Reading List",
      visibility: "Public",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateListSchema>) => {
    startTransition(async () => {
      const res = await create_list(values);
      if (res) setLists([res, ...lists]);
    });
  };

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasNextPage,
    onLoadMore: () => {
      setIsLoading(true);
      get_lists({
        user_id: data?.user_id,
        cursor: {
          cursor,
          limit: 5,
        },
      }).then((data) => {
        setLists([...lists, ...data.items]);
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
    <Popover>
      <PopoverTrigger asChild>
        <Button variant={"ghost"} size={"icon"}>
          {bookmark?.length == 0 ? <BookmarkIcon /> : <BookmarkFilledIcon />}
        </Button>
      </PopoverTrigger>
      <PopoverContent>
        <div className="grid gap-4">
          <div className="space-y-2">
            <h4 className="font-medium leading-none">Save</h4>
            <p className="text-sm text-muted-foreground">Choose the list</p>
          </div>
          <div className="h-36 flex flex-col gap-3">
            {lists.map((list) => (
              <div key={list.id} className="flex items-center gap-1">
                <Checkbox
                  id={list.id}
                  checked={bookmark.map((list) => list.id).includes(list.id)}
                  onCheckedChange={(checked) => {
                    if (checked) {
                      add_article(list.id, article.id);
                      setBookmark([...bookmark, list]);
                    } else {
                      remove_article(list.id, article.id);
                      setBookmark(bookmark.filter((l) => l.id != list.id));
                    }
                  }}
                />
                <label
                  htmlFor={list.id}
                  className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                  {list.label}
                </label>
              </div>
            ))}
            {(isLoading || hasNextPage) && (
              <div className="w-full h-5" ref={ref} />
            )}
          </div>
          <div className="grid gap-2">
            <Dialog>
              <DialogTrigger asChild>
                <Button variant={"secondary"}>
                  <PlusIcon className="mr-2" /> Create List
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Create List</DialogTitle>
                </DialogHeader>
                <Form {...CreateListForm}>
                  <form
                    id="create_list"
                    className="grid grid-2 py-4"
                    onSubmit={CreateListForm.handleSubmit(onSubmit)}
                  >
                    <FormField
                      control={CreateListForm.control}
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
                    <FormField
                      control={CreateListForm.control}
                      name="visibility"
                      render={({ field }) => (
                        <FormItem className="grid grid-cols-4 items-center gap-4">
                          <FormLabel>Visibility</FormLabel>
                          <Select
                            onValueChange={field.onChange}
                            defaultValue={field.value}
                          >
                            <FormControl>
                              <SelectTrigger className="col-span-3">
                                <SelectValue />
                              </SelectTrigger>
                            </FormControl>
                            <SelectContent>
                              <SelectItem value="Public">Public</SelectItem>
                              <SelectItem value="Private">Private</SelectItem>
                              <SelectItem value="Bylink">By link</SelectItem>
                            </SelectContent>
                          </Select>
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
                    <Button form="create_list" type="submit">
                      Create List
                    </Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  );
};

export default ListPopover;
