"use client";

import { create_list, get_lists } from "@/app/actions/list";
import ListList from "@/components/list/list/list";
import { Button } from "@/components/ui/button";
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
} from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";
import { CreateListSchema } from "@/lib/definitions";
import { List } from "@/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { ListPlusIcon } from "lucide-react";
import React, { useState, useTransition } from "react";
import { useForm } from "react-hook-form";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { z } from "zod";

const ListsPage = () => {
  const { data: user, status } = useSession({
    authenticated: true,
  });

  const [lists, setLists] = useState<List[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [pending, startTransition] = useTransition();

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
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_lists({
        user_id: user!.user_id,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setLists([...lists, ...data.items]);
        if (data.next_cursor !== null) {
          setCursor(data.next_cursor);
        } else {
          setHasMore(false);
        }
      });
      setIsLoading(false);
    },
  });

  if (status == "loading") return <Skeleton className="w-full h-full" />;

  return (
    <div className="w-full">
      <div className="p-4">
        <h1>Lists</h1>
      </div>
      <Separator className="w-full" orientation="horizontal" />
      <div className="p-4">
        <div className="flex flex-col gap-4">
          <Dialog>
            <DialogTrigger asChild>
              <Button
                variant={"ghost"}
                className="w-full h-auto rounded-xl p-6 border text-card-foreground shadow border-dashed"
              >
                <h3 className="flex gap-2 justify-center items-center font-semibold leading-none tracking-tight text-muted-foreground">
                  <ListPlusIcon className="h-7 w-7" />
                  Create List
                </h3>
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
                  <Button disabled={pending} form="create_list" type="submit">
                    Create List
                  </Button>
                </DialogClose>
              </DialogFooter>
            </DialogContent>
          </Dialog>
          <ListList
            onDelete={(id) => {
              setLists(lists.filter((list) => list.id !== id));
            }}
            deletable
            editable
            lists={lists}
          />
          {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
        </div>
      </div>
    </div>
  );
};

export default ListsPage;
