'use client';

import Editor from './_components/Editor';

const ArticlePage = () => {
  return (
    <div className='flex h-full w-full flex-col overflow-hidden p-4'>
      <div className='shrink-0 px-3 py-2'>
        Article Title
      </div>
      <Editor className='min-h-0 shrink grow' />
    </div>
  );
};

export default ArticlePage;
