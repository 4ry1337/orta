import ArticleList from '@/components/articles/ArticleList';
import { Skeleton } from '@/components/ui/skeleton';
import { Suspense } from 'react';
import CreateArticleButton from './_components/CreateArticleButton';

const WritePage = () => {
  return (
    <div className='mt-8 flex w-full flex-col'>
      <div className='flex flex-row items-center px-4 py-2'>
        <div className='grow'>
          <h1>Your articles</h1>
        </div>
        <CreateArticleButton />
      </div>
      <div className='space-y-4 px-4'>
        <Suspense fallback={<Skeleton />}>
          <ArticleList user_id={1} />
        </Suspense>
      </div>
    </div>
  );
};

export default WritePage;
