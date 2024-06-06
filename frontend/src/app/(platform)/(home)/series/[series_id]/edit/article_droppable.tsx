import { FullArticle } from "@/lib/types";
import { useDroppable } from "@dnd-kit/core";
import { HTMLAttributes } from "react";
import ArticleDraggableCard from "./article_draggable";

interface DropableProps extends HTMLAttributes<HTMLDivElement> {
  articles: FullArticle[];
  id: string;
}

const ArticleDroppable = ({ id, articles, ...props }: DropableProps) => {
  const { isOver, setNodeRef } = useDroppable({
    id,
  });

  return (
    <div ref={setNodeRef} className="flex flex-col gap-4 h-full">
      {articles.map((article) => (
        <ArticleDraggableCard key={article.id} article={article} />
      ))}
    </div>
  );
};

export default ArticleDroppable;
