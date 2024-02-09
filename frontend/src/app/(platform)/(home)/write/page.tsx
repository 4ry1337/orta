import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from '@/components/ui/card';
import { getSession } from 'next-auth/react';

async function getArticles(id: number) {
  // TODO: change url
  const res = await fetch(
    `http:/localhost:5000/api/article`,
    {
      method: 'GET',
      body: JSON.stringify({
        users: [id],
      }),
    }
  );

  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error('Failed to fetch data');
  }

  return res.json();
}

const WritePage = async () => {
  const session = await getSession();
  if (!session) {
    return <div></div>;
  }
  const articles = await getArticles(
    Number(session.user.id)
  );
  return (
    <div className='flex flex-wrap'>
      {articles.map((article: { title: string }) => {
        <Card>
          <CardHeader>
            <CardHeader>{article.title}</CardHeader>
          </CardHeader>
          <CardContent></CardContent>
          <CardFooter></CardFooter>
        </Card>;
      })}
    </div>
  );
};

export default WritePage;
