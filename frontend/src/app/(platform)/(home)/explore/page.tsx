"use client";
import { get_articles } from "@/app/actions/article";
import { get_lists } from "@/app/actions/list";
import { get_serieses } from "@/app/actions/series";
import { get_users } from "@/app/actions/user";
import ArticleTab from "@/components/article/article_tab";
import ArticleList from "@/components/article/list/list";
import ListList from "@/components/list/list/list";
import ListTab from "@/components/list/list_tab";
import SeriesList from "@/components/series/series/list";
import SeriesTab from "@/components/series/series_tab";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import UserList from "@/components/user/list/list";
import { FullArticle, FullUser, List, Series } from "@/lib/types";
import debounce from "lodash.debounce";
import { Search, XIcon } from "lucide-react";
import Link from "next/link";
import { useEffect, useMemo, useState } from "react";
import useInfiniteScroll from "react-infinite-scroll-hook";

const ExplorePage = () => {
  const [query, setQuery] = useState("");

  const [limit, setLimit] = useState(10);

  const [articles, setArticles] = useState<FullArticle[]>([]);

  const [articlesCursor, setArticlesCursor] = useState<string | undefined>(
    undefined,
  );

  const [hasMoreArticles, setHasMoreArticles] = useState(true);

  const [isArticlesLoading, setIsArticlesLoading] = useState(false);

  const [articleRef] = useInfiniteScroll({
    loading: isArticlesLoading,
    hasNextPage: hasMoreArticles,
    onLoadMore: () => {
      setIsArticlesLoading(true);
      get_articles(query, {
        cursor: articlesCursor,
        limit,
      }).then((data) => {
        setArticles([...articles, ...data.items]);
        if (data.next_cursor !== null) {
          setArticlesCursor(data.next_cursor);
        } else {
          setHasMoreArticles(false);
        }
      });
      setIsArticlesLoading(false);
    },
  });

  const [series, setSeries] = useState<Series[]>([]);

  const [seriesCursor, setSeriesCursor] = useState<string | undefined>(
    undefined,
  );

  const [hasMoreSeries, setHasMoreSeries] = useState(true);

  const [isSeriesLoading, setIsSeriesLoading] = useState(false);

  const [seriesRef] = useInfiniteScroll({
    loading: isSeriesLoading,
    hasNextPage: hasMoreSeries,
    onLoadMore: () => {
      setIsArticlesLoading(true);
      get_serieses(query, {
        cursor: seriesCursor,
        limit,
      }).then((data) => {
        setSeries([...series, ...data.items]);
        if (data.next_cursor !== null) {
          setSeriesCursor(data.next_cursor);
        } else {
          setHasMoreSeries(false);
        }
      });
      setIsSeriesLoading(false);
    },
  });

  const [lists, setLists] = useState<List[]>([]);

  const [listsCursor, setListsCursor] = useState<string | undefined>(undefined);

  const [hasMoreLists, setHasMoreLists] = useState(true);

  const [isListsLoading, setIsListsLoading] = useState(false);

  const [listsRef] = useInfiniteScroll({
    loading: isListsLoading,
    hasNextPage: hasMoreLists,
    onLoadMore: () => {
      setIsListsLoading(true);
      get_lists(query, {
        cursor: listsCursor,
        limit,
      }).then((data) => {
        setLists([...lists, ...data.items]);
        if (data.next_cursor !== null) {
          setListsCursor(data.next_cursor);
        } else {
          setHasMoreLists(false);
        }
      });
      setIsListsLoading(false);
    },
  });

  const [users, setUsers] = useState<FullUser[]>([]);

  const [usersCursor, setUsersCursor] = useState<string | undefined>(undefined);

  const [hasMoreUsers, setHasMoreUsers] = useState(true);

  const [isUsersLoading, setIsUsersLoading] = useState(false);

  const [usersRef] = useInfiniteScroll({
    loading: isUsersLoading,
    hasNextPage: hasMoreLists,
    onLoadMore: () => {
      setIsUsersLoading(true);
      get_users(query, {
        cursor: listsCursor,
        limit,
      }).then((data) => {
        setUsers([...users, ...data.items]);
        if (data.next_cursor !== null) {
          setUsersCursor(data.next_cursor);
        } else {
          setHasMoreUsers(false);
        }
      });
      setIsUsersLoading(false);
    },
  });

  const handleSearch = (value: string) => {
    setQuery(value);

    setArticles([]);
    setUsers([]);
    setLists([]);
    setSeries([]);

    setArticlesCursor(undefined);
    setUsersCursor(undefined);
    setListsCursor(undefined);
    setSeriesCursor(undefined);

    setHasMoreArticles(true);
    setHasMoreUsers(true);
    setHasMoreLists(true);
    setHasMoreSeries(true);
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
      <div className="p-4">
        <Tabs defaultValue={"articles"}>
          <TabsList className="grid grid-cols-4">
            <TabsTrigger value="articles">
              <Link href={"#articles"}>Articles</Link>
            </TabsTrigger>
            <TabsTrigger value="users">
              <Link href={"#users"}>Users</Link>
            </TabsTrigger>
            <TabsTrigger value="series">
              <Link href={"#series"}>Series</Link>
            </TabsTrigger>
            <TabsTrigger value="lists">
              <Link href={"#lists"}>Lists</Link>
            </TabsTrigger>
          </TabsList>
          <TabsContent className="mt-4 space-y-4" value="articles">
            <ArticleList articles={articles} />
            {(isArticlesLoading || hasMoreArticles) && (
              <div className="w-full h-20" ref={articleRef} />
            )}
          </TabsContent>
          <TabsContent className="mt-4 space-y-4" value="users">
            <UserList users={users} />
            {(isUsersLoading || hasMoreUsers) && (
              <div className="w-full h-20" ref={usersRef} />
            )}
          </TabsContent>
          <TabsContent className="mt-4 space-y-4" value="series">
            <SeriesList serieses={series} />
            {(isSeriesLoading || hasMoreSeries) && (
              <div className="w-full h-20" ref={seriesRef} />
            )}
          </TabsContent>
          <TabsContent className="mt-4 space-y-4" value="lists">
            <ListList lists={lists} />
            {(isListsLoading || hasMoreLists) && (
              <div className="w-full h-20" ref={listsRef} />
            )}
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
};

export default ExplorePage;
