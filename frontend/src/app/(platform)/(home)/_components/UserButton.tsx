'use client';
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Separator } from '@/components/ui/separator';
import { cn } from '@/lib/utils';
import {
  ActivityIcon,
  LogOut,
  Settings,
} from 'lucide-react';
import { signOut, useSession } from 'next-auth/react';
import Link from 'next/link';
import { HTMLAttributes } from 'react';

interface UserButtonProps
  extends HTMLAttributes<HTMLDivElement> {}

const UserButton = ({ className }: UserButtonProps) => {
  const session = useSession();

  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant={'outline'}
          className={cn(
            'h-16 gap-x-4 rounded-full p-3',
            className
          )}
        >
          <Avatar className='h-10 w-10'>
            <AvatarImage
              src={session.data?.user.image}
              alt='@avatar'
            />
            <AvatarFallback>
              {session.data?.user.name.at(0)}
            </AvatarFallback>
          </Avatar>
          <div className='hidden grow flex-col items-start xl:inline-flex'>
            <div>{session.data?.user.name}</div>
          </div>
        </Button>
      </PopoverTrigger>
      <PopoverContent align='start'>
        <div className='flex flex-col gap-y-3'>
          <Button
            variant={'secondary'}
            className='justify-start'
            asChild
          >
            <Link
              href={'/activity'}
              prefetch={false}
              className='justify-start'
            >
              <ActivityIcon className='mr-2 h-4 w-4' />
              <div>Activity Log</div>
            </Link>
          </Button>
          <Button
            variant={'secondary'}
            className='justify-start'
            asChild
          >
            <Link
              href={'/settings'}
              prefetch={false}
            >
              <Settings className='mr-2 h-4 w-4' />
              Settigns
            </Link>
          </Button>
          <Separator />
          <Button
            variant={'destructive'}
            className='justify-start'
            onClick={() => signOut()}
          >
            <LogOut className='mr-2 h-4 w-4' />
            Sign Out
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  );
};

export default UserButton;
