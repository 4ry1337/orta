"use client";

import {
  Popover,
  PopoverAnchor,
  PopoverContent,
} from "@/components/ui/popover";
import { Tag } from "@/lib/types";
import useInfiniteScroll from "react-infinite-scroll-hook";
import { get_tags } from "@/app/actions/tags";
import debounce from "lodash.debounce";
import { useEffect, useMemo, useState } from "react";
import { useArticle } from "../page";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";

const TagsForm = () => {
  const { article, update } = useArticle();

  const [isOpen, setIsOpen] = useState(false);

  const [tags, setTags] = useState<Tag[]>(article.tags);

  const [searchtags, setSearchTags] = useState<Tag[]>([]);

  const [query, setQuery] = useState("");

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_tags({
        query: query,
        cursor: {
          cursor,
          limit: 5,
        },
      }).then((data) => {
        setSearchTags([...searchtags, ...data.items]);
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
    if (value != "") {
      setIsOpen(true);
    } else {
      setIsOpen(false);
    }
    setSearchTags([]);
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
    <div>
      <Popover open={isOpen}>
        <PopoverAnchor asChild>
          <div>
            <Label htmlFor="tags">Tags</Label>
            <Input
              id="tags"
              type="text"
              onChange={(e) => debouncedResults(e.target.value)}
              placeholder="Set tags"
            />
          </div>
        </PopoverAnchor>
        <PopoverContent
          className="w-[--radix-popover-trigger-width] max-h-[--radix-popover-content-available-height]"
          onOpenAutoFocus={(e) => e.preventDefault()}
        >
          <ScrollArea>
            <div className="max-h-60 h-min flex flex-col gap-2">
              {searchtags.length !== 0 ? (
                searchtags.map((tag) => (
                  <Button
                    onClick={() => {
                      setTags([...tags, tag]);
                    }}
                    variant={"secondary"}
                    key={tag.slug}
                  >
                    {tag.label} ({tag.article_count})
                  </Button>
                ))
              ) : (
                <div className="w-full h-full text-center">Search tags</div>
              )}
              {(isLoading || hasMore) && (
                <div className="w-full h-20" ref={ref} />
              )}
            </div>
            <ScrollBar />
          </ScrollArea>
        </PopoverContent>
      </Popover>
    </div>
  );
};

export default TagsForm;
