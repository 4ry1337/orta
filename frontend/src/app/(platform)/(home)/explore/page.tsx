"use client";
import { get_articles } from "@/app/actions/article";
import ArticleList from "@/components/article/list/list";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { FullArticle } from "@/lib/types";
import debounce from "lodash.debounce";
import { Search, XIcon } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";

const ExplorePage = () => {
  const [query, setQuery] = useState("");

  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [limit, setLimit] = useState(5);

  const [cursor, setCursor] = useState<string | undefined>(undefined);

  const [hasMore, setHasMore] = useState(true);

  const [isLoading, setIsLoading] = useState(false);

  const [ref] = useInfiniteScroll({
    loading: isLoading,
    hasNextPage: hasMore,
    onLoadMore: () => {
      setIsLoading(true);
      get_articles({
        query: query,
        published: true,
        cursor: {
          cursor,
          limit,
        },
      }).then((data) => {
        setArticles([...articles, ...data.items]);
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
    setArticles([]);
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
    <div className="relative h-32">
      <div className="sticky w-full p-4 space-y-4">
        <div className="relative">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            type="text"
            onChange={(e) => debouncedResults(e.target.value)}
            placeholder="Search"
            className="px-8"
          />
          <XIcon className="absolute right-2 top-2.5 h-4 w-4 text-muted-foreground" />
        </div>
      </div>
      <Separator orientation="horizontal" />
      <div className="p-4 space-y-4">
        <ArticleList articles={articles} />
        {(isLoading || hasMore) && <div className="w-full h-20" ref={ref} />}
      </div>
    </div>
  );
};

export default ExplorePage;
