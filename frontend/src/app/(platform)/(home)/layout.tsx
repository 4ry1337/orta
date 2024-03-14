import { Separator } from '@/components/ui/separator';
import Sidebar from './_components/Sidebar';

const HomeLayout = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <div className='container flex h-full'>
      <Sidebar
        className={`hidden w-24 shrink-0 p-4 sm:block xl:w-80`}
      />
      <Separator
        orientation='vertical'
        className='hidden sm:block'
      />
      <main className='shrink grow overflow-hidden'>
        {children}
      </main>
    </div>
  );
};

export default HomeLayout;
