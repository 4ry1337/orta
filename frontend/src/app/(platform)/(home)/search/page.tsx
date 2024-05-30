"use client";

import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useSearchParams } from "next/navigation";
import { useState } from "react";
import ArticleSearchTab from "./article_search_tab";

const SearchPage = () => {
  const searchParams = useSearchParams();
  const [query, setQuery] = useState("");

  return (
    <div className="py-4 max-w-full">
      <div className="px-4 py-2.5 w-full">
        <Input placeholder="Search" />
      </div>
      <div className="px-4 w-full">
        <Tabs defaultValue="articles">
          <TabsList className="grid grid-cols-4 w-full">
            <TabsTrigger value="articles">Articles</TabsTrigger>
            <TabsTrigger value="users">Users</TabsTrigger>
            <TabsTrigger value="lists">Lists</TabsTrigger>
            <TabsTrigger value="series">Series</TabsTrigger>
          </TabsList>
          <TabsContent value="articles">
            <div>Articles</div>
            <ArticleSearchTab />
          </TabsContent>
          <TabsContent value="users">
            <div>Users</div>
          </TabsContent>
          <TabsContent value="lists">
            <div>Lists</div>
          </TabsContent>
          <TabsContent value="series">
            <div>Series</div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
};

export default SearchPage;
