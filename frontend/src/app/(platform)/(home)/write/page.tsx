'use server'
import { authOptions } from '@/app/api/auth/[...nextauth]/route';
import ArticleList from '@/components/articles/ArticleList';
import { getServerSession } from 'next-auth';
import CreateArticleButton from './_components/CreateArticleButton';
import { toast } from 'sonner';
import db from '@/lib/prismadb';
import { Article } from '@prisma/client';

async function getArticles(id: string): Promise<Article[]> {
  // TODO: change url
  // return fetch(`http://localhost:5000/api/article/search`, {
  //   cache: 'no-store',
  //   next: {
  //     tags: ["article", id.toString()],
  //   },
  //   method: 'POST',
  //   headers: {
  //     Accept: 'application/json',
  //     'Content-Type': 'application/json',
  //   },
  //   body: JSON.stringify({
  //     users: [id],
  //   }),
  // }).then(async (res) => {
  //   const response = await res.json();
  //   return response;
  // }).catch(error => {
  //   console.log(error);
  // })
  //
  return await db.article.findMany({
    where: {
      userIds: {
        has: id
      }
    }
  })
}

const WritePage = async () => {
  const session = await getServerSession(authOptions);
  const articles = await getArticles(session!.user.id);
  return (
    <div className='flex h-full flex-col gap-4 px-4'>
      <div className='flex flex-row items-center pt-4'>
        <div className='grow'>
          <h1>Your articles</h1>
        </div>
        <CreateArticleButton user_id={session!.user.id} />
      </div>
      <ArticleList
        articles={articles}
        className='h-full overflow-hidden'
      />
    </div>
  );
};

export default WritePage;
