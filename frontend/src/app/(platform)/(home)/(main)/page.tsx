import { Button } from '@/components/ui/button';
import { Plus } from 'lucide-react';
import Header from '../_components/Header';

const HomePage = async () => {
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
          {/*interests?.map((interest) => {
            return (
              <h3
                className=''
                key={interest}
              >
                {interest}
              </h3>
            );
          }) */}
        </div>
      </Header>
      <div className='grow'>
        <div>Not implemented</div>
      </div>
    </>
  );
};

export default HomePage;
