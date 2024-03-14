'use client';
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
import { Button } from '../ui/button';
import { useAction } from '@/hooks/useAction';
import { deletearticle } from '@/app/actions/article/delete';
import { useToast } from '../ui/use-toast';
import { Article } from '@prisma/client';

interface ArticleItemProps
  extends HTMLAttributes<HTMLDivElement> {
  article: Article;
}

const ArticleItem = ({
  article,
  ...props
}: ArticleItemProps) => {
  const { toast } = useToast();
  const { execute } = useAction(deletearticle, {
    onError: (error) => {
      toast({
        variant: 'destructive',
        title: error.status,
        description: error.message,
      });
    },
    onSuccess(data) {
      toast({
        title: data,
      });
    },
    onComplete() {
      
    },
  });
  const DeleteArticle = () => {
    execute({article_id: article.id})
    //router.push(`/${1}/${1}`);
  };
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          <Link
            href={`${article.userIds[0]}/${article.id}`}
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
        <div className='flex flex-row justify-between gap-2'>
        <div className='flex flex-wrap grow'>
          {article.tag_list.map((tag) => (
            <Badge
              key={tag}
              variant={'default'}
            >
              {tag}
            </Badge>
          ))}
        </div>
          {/*<Button onClick={()=> DeleteArticle()}>
            Delete
          </Button> */}
        </div>
      </CardFooter>
    </Card>
  );
};

export default ArticleItem;
