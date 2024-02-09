import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Separator } from '@/components/ui/separator';
import Link from 'next/link';

const MainLayout = async ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const tags = [
    'dev',
    'design',
    'ui/ux',
    '3D',
    'blender',
    'music',
    'audio',
  ];
  const top_authors = [
    {
      id: 1,
      image: null,
      initials: 'RY',
      name: 'Rakhat Yskak',
    },
  ];
  const marketing = [
    {
      href: '/tos',
      label: 'Terms of Service',
    },
    {
      href: '/about',
      label: 'About',
    },
    {
      href: '/privacy',
      label: 'Privacy Policy',
    },
    {
      href: '/cookie',
      label: 'Cookie Policy',
    },
  ];
  return (
    <div className='flex w-full grow flex-row '>
      <div className='flex grow flex-col'>{children}</div>
      <Separator
        orientation='vertical'
        className='hidden lg:block'
      />
      <div className='hidden w-80 p-4 lg:block'>
        <div className='flex h-full flex-col gap-4'>
          <Input
            type='search'
            placeholder='Search'
          />
          <div className=''>
            <h2 className='mb-4'>Trending</h2>
            <div className='inline-flex flex-wrap gap-2'>
              {tags.map((tag) => {
                return (
                  <Link
                    href={`/search?tags=${tag}`}
                    key={tag}
                    className=''
                  >
                    <Badge>{tag}</Badge>
                  </Link>
                );
              })}
            </div>
          </div>
          <div className=''>
            <h2 className='mb-4'>Popular Writers</h2>
            <div className='flex flex-col gap-2'>
              {top_authors.map((top_author) => {
                return (
                  <Link
                    href={`/@${top_author.id}`}
                    key={top_author.id}
                    className='inline-flex flex-row items-center'
                  >
                    <Avatar className='h-10 w-10'>
                      <AvatarImage
                        src={top_author.image}
                        alt='@avatar'
                      />
                      <AvatarFallback>
                        {top_author.initials}
                      </AvatarFallback>
                    </Avatar>
                    <div className='mx-4 grow'>
                      {top_author.name}
                    </div>
                  </Link>
                );
              })}
            </div>
          </div>
          <div className='inline-flex flex-wrap gap-4'>
            {marketing.map((marketing_page) => {
              return (
                <Link
                  key={marketing_page.href}
                  href={marketing_page.href}
                >
                  {marketing_page.label}
                </Link>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
};

export default MainLayout;
