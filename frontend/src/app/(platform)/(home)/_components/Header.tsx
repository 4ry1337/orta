import { cn } from '@/lib/utils';
import { HTMLAttributes } from 'react';

interface HeaderProps
  extends HTMLAttributes<HTMLDivElement> {}

const Header = ({
  children,
  className,
  ...props
}: HeaderProps) => {
  return (
    <div
      className={cn(
        'h-18 inline-flex flex-row border-b',
        className
      )}
    >
      {children}
    </div>
  );
};

export default Header;
