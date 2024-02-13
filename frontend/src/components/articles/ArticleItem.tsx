'use client';
import { Article } from '@/types';
import Link from 'next/link';
import { HTMLAttributes } from 'react';
import { Badge } from '../ui/badge';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '../ui/card';

interface ArticleItemProps
  extends HTMLAttributes<HTMLDivElement> {
  article: Article;
}

const ArticleItem = ({
  article,
  ...props
}: ArticleItemProps) => {
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          <Link
            href={`${article.user_ids[0]}/${article.id}`}
          >
            {article.title || 'No title'}
          </Link>
        </CardTitle>
        <CardDescription></CardDescription>
      </CardHeader>
      <CardContent>
        <div className='flex flex-row gap-2'>
          <div>Created at:</div>
          <time>
            {new Date(article.created_at).toLocaleString()}
          </time>
        </div>
        <div className='flex flex-row gap-2'>
          <div>Last Updated:</div>
          <time>
            {new Date(article.updated_at).toLocaleString()}
          </time>
        </div>
        <div className='flex flex-row gap-2'>
          <div>Published:</div>
          {!article.published_at ? (
            <div>Unpublished</div>
          ) : (
            <time>
              {new Date(
                article.published_at
              ).toLocaleString()}
            </time>
          )}
        </div>
      </CardContent>
      <CardFooter>
        <div className='flex flex-wrap'>
          {article.tag_list.map((tag) => (
            <Badge
              key={tag}
              variant={'default'}
            >
              {tag}
            </Badge>
          ))}
        </div>
        <div>
          {article.user_ids.map((id) => (
            <div key={id}>{id}</div>
          ))}
        </div>
      </CardFooter>
    </Card>
  );
};

export default ArticleItem;
