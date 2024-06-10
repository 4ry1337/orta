"use client";

import { add_author, publish, update_article } from "@/app/actions/article";
import { get_users } from "@/app/actions/user";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { FullUser } from "@/lib/types";
import debounce from "lodash.debounce";
import { Search, XIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";
import {
  Popover,
  PopoverAnchor,
  PopoverContent,
} from "@/components/ui/popover";
import { useArticle } from "@/context/article_context";

const CollaboratorForm = () => {
  const { article, update } = useArticle();

  const [query, setQuery] = useState("");

  const [users, setUsers] = useState<FullUser[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [disabled, setDisabled] = useState(true);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    disabled,
    onLoadMore: () => {
      setIsLoading(true);
      get_users(query, {
        cursor,
        limit,
      }).then((data) => {
        setUsers([
          ...users,
          ...data.items.filter(
            (u) => !article.users.map((e) => e.id).includes(u.id),
          ),
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

  const [isOpen, setIsOpen] = useState(false);

  const handleSearch = (value: string) => {
    setQuery(value);
    if (value != "") {
      setDisabled(false);
    } else {
      setDisabled(true);
    }
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
    <div className="space-y-8">
      <Popover open={isOpen}>
        <PopoverAnchor asChild>
          <div className="w-full relative">
            <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input
              type="text"
              onChange={(e) => debouncedResults(e.target.value)}
              onFocus={() => setIsOpen(true)}
              onBlur={() => setIsOpen(false)}
              placeholder="Search by Username"
              className="px-8"
            />
            <XIcon className="absolute right-2 top-2.5 h-4 w-4 text-muted-foreground" />
          </div>
        </PopoverAnchor>
        <PopoverContent
          onOpenAutoFocus={(e) => e.preventDefault()}
          className="w-[--radix-popover-trigger-width] max-h-[--radix-popover-content-available-height]"
        >
          <div className="space-y-4">
            {users.length !== 0 ? (
              users.map((user) => (
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
                        update({ ...article, users: [...article.users, user] });
                      });
                    }}
                  >
                    Add Collaborator
                  </Button>
                </div>
              ))
            ) : (
              <div className="w-full h-full text-center">
                Search by Username
              </div>
            )}
            {(isLoading || hasMore) && (
              <div className="w-full h-20" ref={ref} />
            )}
          </div>
        </PopoverContent>
      </Popover>
      <div className="space-y-2">
        <h4>Collaborators:</h4>
        <div className="space-y-2">
          {article.users.map((user) => (
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
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default CollaboratorForm;
