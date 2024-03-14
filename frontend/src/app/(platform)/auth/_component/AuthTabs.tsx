'use client';

import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/components/ui/tabs';
import { useSession } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { HTMLAttributes, useEffect } from 'react';
import SignInForm from './SingInForm';
import SignUpForm from './SingUpForm';

interface AuthTabsProps
  extends HTMLAttributes<HTMLDivElement> {}

const AuthTabs = (props: AuthTabsProps) => {
  const router = useRouter();
  const session = useSession();
  useEffect(() => {
    if (session.status === 'authenticated') {
      console.log('authenticated');
      router.push('/');
    }
  }, [session?.status, router]);
  return (
    <Tabs defaultValue='signin'>
      <TabsList className='grid w-full grid-cols-2'>
        <TabsTrigger value='signin'>Sign In</TabsTrigger>
        <TabsTrigger value='signup'>Sign Up</TabsTrigger>
      </TabsList>
      <TabsContent value='signin'>
        <SignInForm />
      </TabsContent>
      <TabsContent value='signup'>
        <SignUpForm />
      </TabsContent>
    </Tabs>
  );
};

export default AuthTabs;
