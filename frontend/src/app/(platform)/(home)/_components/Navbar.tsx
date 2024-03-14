import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from '@/components/ui/avatar';
import { cn } from '@/lib/utils';
import { HTMLAttributes } from 'react';

interface NavbarProps
  extends HTMLAttributes<HTMLDivElement> {}

const Navbar = ({ className }: NavbarProps) => {
  return (
    <header className={cn('flex', className)}>
      <Avatar>
        <AvatarImage
          src={'https://github.com/shadcn.png'}
          alt='@avatar'
        />
        <AvatarFallback>RY</AvatarFallback>
      </Avatar>
    </header>
  );
};
export default Navbar;
