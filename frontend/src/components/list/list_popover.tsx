import React, { useState } from "react";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { Button } from "../ui/button";
import { BookmarkIcon } from "@radix-ui/react-icons";
import CreateListDialog from "./create_list_dialog";
import useSWR from "swr";
import { get_lists } from "@/app/actions/list";
import { useSession } from "@/context/session_context";
import { ScrollArea } from "../ui/scroll-area";

const ListPopover = () => {
  const { status, data } = useSession({
    authenticated: true,
  });

  const [page, setPage] = useState(1);

  const pages: React.ReactNode[] = [];

  const { data: lists } = useSWR(
    status == "loading"
      ? null
      : {
        usernames: data.username,
        pagination: {
          page: page,
          per_page: 10,
        },
      },
    get_lists,
  );

  for (let i = 0; i < page; i++) {
    pages.push(<div key={i}></div>);
  }

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
            <ScrollArea>{pages}</ScrollArea>
          </div>
          <div className="grid gap-2">
            <CreateListDialog />
          </div>
        </div>
      </PopoverContent>
    </Popover>
  );
};

export default ListPopover;
