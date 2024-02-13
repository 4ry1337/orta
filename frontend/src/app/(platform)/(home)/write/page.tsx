import { authOptions } from '@/app/api/auth/[...nextauth]/route';
import ArticleList from '@/components/articles/ArticleList';
import { Article } from '@/types';
import { getServerSession } from 'next-auth';
import CreateArticleButton from './_components/CreateArticleButton';

async function getArticles(id: number): Promise<Article[]> {
  // TODO: change url
  return fetch(`http://localhost:5000/api/article/search`, {
    next: {
      tags: [],
    },
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

const WritePage = async () => {
  const session = await getServerSession(authOptions);
  const articles = await getArticles(
    Number(session!.user.id)
  );
  console.log(articles);
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
