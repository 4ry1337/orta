import ArticleTab from "@/components/article/article_tab";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Search, XIcon } from "lucide-react";

const ExplorePage = () => {
  return (
    <div className="relative h-32">
      <div className="sticky w-full p-4 space-y-4">
        <div className="relative">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input placeholder="Search" className="px-8" />
          <XIcon className="absolute right-2 top-2.5 h-4 w-4 text-muted-foreground" />
        </div>
      </div>
      <Separator orientation="horizontal" />
      <div className="p-4 space-y-4">
        <ArticleTab />
      </div>
    </div>
  );
};

export default ExplorePage;
