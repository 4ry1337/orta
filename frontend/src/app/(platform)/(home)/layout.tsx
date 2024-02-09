import { Separator } from '@/components/ui/separator';
import Sidebar from './_components/Sidebar';

const HomeLayout = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <div className='h-full w-full'>
      <div className='container flex h-full'>
        <Sidebar
          className={`hidden w-20 px-2 py-4 sm:block xl:w-80 xl:px-4`}
        />
        <Separator
          orientation='vertical'
          className='hidden sm:block'
        />
        <main className='flex shrink grow flex-col items-start'>
          {children}
        </main>
      </div>
    </div>
  );
};

export default HomeLayout;
