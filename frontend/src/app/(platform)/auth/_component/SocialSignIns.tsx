'use client';

import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { cn } from '@/lib/utils';
import { Chrome, Github } from 'lucide-react';
import { signIn } from 'next-auth/react';
import { HTMLAttributes, useState } from 'react';

interface SocialSignInsProps
  extends HTMLAttributes<HTMLDivElement> {}

const SocailSignIns = ({
  className,
  ...props
}: SocialSignInsProps) => {
  const [isLoading, setIsLoading] = useState(false);
  const { toast } = useToast();
  const socialAction = (action: string) => {
    setIsLoading(true);
    signIn(action, { redirect: false })
      .then((callback) => {
        if (callback?.error) {
          toast({
            variant: 'destructive',
            title: 'Invalid Credentials',
          });
        }
        if (callback?.ok && !callback?.error) {
          toast({
            title: 'Signed In',
          });
        }
      })
      .finally(() => setIsLoading(false));
  };

  return (
    <div className={cn('flex gap-2', className)}>
      <Button
        className='w-full'
        disabled={isLoading}
        onClick={() => socialAction('github')}
      >
        <Github />
        <span className='ml-2 text-sm font-medium'>
          Github
        </span>
      </Button>
      <Button
        className='w-full'
        disabled={isLoading}
        onClick={() => socialAction('github')}
      >
        <Chrome />
        <span className='ml-2 text-sm font-medium'>
          Google
        </span>
      </Button>
    </div>
  );
};

export default SocailSignIns;
