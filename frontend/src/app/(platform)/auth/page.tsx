import LogoIcon from '@/components/logo';
import AuthTabs from './_component/AuthTabs';

const AuthPage = () => {
  return (
    <div className='container h-full md:grid lg:max-w-none lg:grid-cols-2 lg:px-0'>
      <div className='hidden h-full flex-col bg-muted p-10 dark:border-r lg:flex'>
        <div className='flex items-center text-lg font-medium'>
          <LogoIcon />
          <span className='ml-2'>Orta</span>
        </div>
        <div className='mt-auto'>
          <blockquote className='space-y-2'>
            <p className='text-lg'>
              &ldquo;A platform for creative professionals
              to create articles and effectively interact
              with audiences online.&rdquo;
            </p>
            <footer className='text-sm'>
              Orta Developers
            </footer>
          </blockquote>
        </div>
      </div>
      <div className='py-10 lg:p-10'>
        <div className='mx-auto flex h-full max-w-96'>
          <AuthTabs />
        </div>
      </div>
    </div>
  );
};

export default AuthPage;
