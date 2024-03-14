import { Button } from '@/components/ui/button';
import { Plus } from 'lucide-react';
import { getServerSession } from 'next-auth';
import Header from '../_components/Header';
import db from '@/lib/prismadb';

const getInterests = async (id: string) => {
  return (await db.profile.findFirst({
    where: {
      userId: id,
    },
    select: {
      interests: true
    }
  }))?.interests
}

const HomePage = async () => {
  const session = await getServerSession();

  const interests = await getInterests(session!.user.id);
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
          {interests?.map((interest) => {
            return (
              <h3
                className=''
                key={interest}
              >
                {interest}
              </h3>
            );
          })}
        </div>
      </Header>
      <div className='grow'>
        <div>Not implemented</div>
      </div>
    </>
  );
};

export default HomePage;
