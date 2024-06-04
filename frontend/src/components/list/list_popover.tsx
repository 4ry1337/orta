import React, { useState, useTransition } from "react";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { Button } from "../ui/button";
import { BookmarkIcon, PlusIcon } from "@radix-ui/react-icons";
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
import { List } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { create_list, get_lists } from "@/app/actions/list";
import ListList from "./list/list";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { CreateListSchema } from "@/lib/definitions";
import { zodResolver } from "@hookform/resolvers/zod";
import SelectListList from "./select_list/list";

const ListPopover = ({ article_id }: { article_id: string }) => {
  const { status, data } = useSession({
    authenticated: true,
  });

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
          <BookmarkIcon />
        </Button>
      </PopoverTrigger>
      <PopoverContent>
        <div className="grid gap-4">
          <div className="space-y-2">
            <h4 className="font-medium leading-none">Save</h4>
            <p className="text-sm text-muted-foreground">Choose the list</p>
          </div>
          <div className="h-36">
            <SelectListList article_id={article_id} lists={lists} />
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
                    id="create_article"
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
                  <Button form="create_article" type="submit">
                    Create List
                  </Button>
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
