import { Article } from '@/types';
import { HTMLAttributes } from 'react';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
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
        <CardHeader>{article.title}</CardHeader>
      </CardHeader>
      <CardContent></CardContent>
      <CardFooter></CardFooter>
    </Card>
  );
};

export default ArticleItem;
