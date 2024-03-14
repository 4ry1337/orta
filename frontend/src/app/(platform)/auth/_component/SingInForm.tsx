'use client';

import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { toast, useToast } from '@/components/ui/use-toast';
import { zodResolver } from '@hookform/resolvers/zod';
import { signIn } from 'next-auth/react';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import AuthFooter from './AuthFooter';
import SocailSignIns from './SocialSignIns';

const signinSchema = z.object({
  email: z.string().email().min(1, {
    message: 'Email is required.',
  }),
  password: z.string().min(1, {
    message: 'Password is required.',
  }),
});

const SignInForm = () => {
  const [isLoading, setIsLoading] = useState(false);
  const signInForm = useForm<z.infer<typeof signinSchema>>({
    resolver: zodResolver(signinSchema),
    defaultValues: {
      email: '',
      password: '',
    },
  });
  function onSubmit(values: z.infer<typeof signinSchema>) {
    setIsLoading(true);
    signIn('credentials', { ...values, redirect: false })
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
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Login</CardTitle>
        <CardDescription>
          Enter email and password below to access your
          account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...signInForm}>
          <form
            onSubmit={signInForm.handleSubmit(onSubmit)}
            className='space-y-8'
          >
            <FormField
              control={signInForm.control}
              name='email'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Email</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={signInForm.control}
              name='password'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input
                      type='password'
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div>
              <Button
                disabled={isLoading}
                className='w-full'
                type='submit'
              >
                Sign In
              </Button>
            </div>
          </form>
        </Form>
        <div className='mt-6'>
          <div className='relative'>
            <div className='absolute inset-0 flex items-center'>
              <div className='w-full border-t border-muted-foreground' />
            </div>
            <div className='relative flex justify-center text-center text-sm text-muted-foreground'>
              <span className='bg-background px-2'>
                Or continue with
              </span>
            </div>
          </div>
          <SocailSignIns className={'mt-6'} />
        </div>
      </CardContent>
      <CardFooter>
        <AuthFooter />
      </CardFooter>
    </Card>
  );
};

export default SignInForm;
