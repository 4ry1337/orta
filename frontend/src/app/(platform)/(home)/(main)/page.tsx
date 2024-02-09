import { Button } from '@/components/ui/button';
import { Plus } from 'lucide-react';
import { getServerSession } from 'next-auth';
import Header from '../_components/Header';

const HomePage = async () => {
  const user = await getServerSession();
  const tag_filter = [
    'dev',
    'design',
    '3d',
    'ui/ux',
    'system design',
  ];
  return (
    <>
      <Header className=''>
        <Button
          size={'icon'}
          variant={'ghost'}
        >
          <Plus />
        </Button>
        <div className='inline-flex flex-row gap-4'>
          {tag_filter.map((tag) => {
            return (
              <h3
                className=''
                key={tag}
              >
                {tag}
              </h3>
            );
          })}
        </div>
      </Header>
      <div className='grow'>Home Page</div>
    </>
  );
};

export default HomePage;
