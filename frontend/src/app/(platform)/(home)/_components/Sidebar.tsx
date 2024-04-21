import LogoIcon from '@/components/logo';
import { ModeToggle } from '@/components/mode-toggle';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import {
  BellIcon,
  Home,
  Library,
  Pen,
  Search,
} from 'lucide-react';
import Link from 'next/link';
import React from 'react';

interface SidebarProps
  extends React.HTMLAttributes<HTMLDivElement> { }

interface IRoute {
  label: string;
  icon: any;
  href: string;
}

const Sidebar = ({ className }: SidebarProps) => {
  const routes: IRoute[] = [
    {
      label: 'Home',
      href: '/',
      icon: Home,
    },
    {
      label: 'Search',
      href: '/search',
      icon: Search,
    },
    {
      label: 'Notifications',
      href: '/notifications',
      icon: BellIcon,
    },
    {
      label: 'Lists',
      href: '/lists',
      icon: Library,
    },
  ];
  return (
    <header className={cn('', className)}>
      <aside className='flex h-full flex-col gap-4'>
        <div className='inline-flex flex-col items-center justify-center gap-2 xl:flex-row'>
          <Button
            variant={'ghost'}
            asChild
            className='h-16 grow gap-x-4 rounded-full p-3'
          >
            <Link
              href={'/'}
              prefetch={false}
              className=''
            >
              <LogoIcon />
              <h3 className='hidden grow xl:block'>Orta</h3>
            </Link>
          </Button>
          <ModeToggle
            variant={'ghost'}
            size={'icon'}
            className='rounded-full'
          />
        </div>
        <Button
          asChild
          className='h-16 gap-x-4 rounded-full p-3'
        >
          <Link
            href={'/write'}
            prefetch={false}
          >
            <Pen className='' />
            <h3 className='hidden xl:block'>Write</h3>
          </Link>
        </Button>
        <div className='inline-flex grow flex-col items-start gap-2 px-1 xl:items-stretch'>
          {routes.map((route) => {
            return (
              <Button
                key={route.label}
                variant={'ghost'}
                asChild
                className='h-14 gap-x-4 rounded-full p-4'
              >
                <Link
                  href={route.href}
                  prefetch={false}
                >
                  <route.icon />
                  <h3 className='hidden grow xl:block'>
                    {route.label}
                  </h3>
                </Link>
              </Button>
            );
          })}
        </div>
      </aside>
    </header>
  );
};

// <UserButton className='' />
export default Sidebar;
