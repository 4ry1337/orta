import { Article } from '@/types';
import { HTMLAttributes } from 'react';
import { ScrollArea } from '../ui/scroll-area';
import ArticleItem from './ArticleItem';

async function getArticles(id: number): Promise<Article[]> {
  // TODO: change url
  return fetch(`http://localhost:5000/api/article/search`, {
    method: 'POST',
    headers: {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      users: [id],
    }),
  }).then(async (res) => {
    const response = await res.json();
    return response;
  });
}

interface ArticleListProps
  extends HTMLAttributes<HTMLDivElement> {
  user_id: number;
}

const ArticleList = async ({
  user_id,
  ...props
}: ArticleListProps) => {
  const articles = await getArticles(1);
  return (
    <ScrollArea>
      <div className='flex flex-col'>
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
