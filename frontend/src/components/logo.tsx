import { cn } from '@/lib/utils';
import Image from 'next/image';
import { HTMLAttributes } from 'react';
import OrtaLogo from '../../public/orta.svg';
interface LogoProps
  extends HTMLAttributes<HTMLImageElement> {}

const LogoIcon = ({ className }: LogoProps) => {
  return (
    <div
      className={cn(
        'relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full',
        className
      )}
    >
      <Image
        priority
        className='aspect-square h-full w-full invert dark:invert-0'
        src={OrtaLogo}
        alt={'Orta'}
        fill
      />
    </div>
  );
};

export default LogoIcon;
