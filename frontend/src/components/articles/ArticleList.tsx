'use client';
import { cn } from '@/lib/utils';
import { Article } from '@/types';
import { HTMLAttributes } from 'react';
import { ScrollArea } from '../ui/scroll-area';
import ArticleItem from './ArticleItem';

interface ArticleListProps
  extends HTMLAttributes<HTMLDivElement> {
  articles: Article[];
}

const ArticleList = ({
  articles,
  className,
  ...props
}: ArticleListProps) => {
  return (
    <ScrollArea className={cn('pr-4', className)}>
      <div className='flex flex-col gap-2'>
        {articles?.map((article: Article) => (
          <ArticleItem
            key={article.id}
            article={article}
          />
        ))}
      </div>
    </ScrollArea>
  );
};

export default ArticleList;
