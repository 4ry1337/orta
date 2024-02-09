'use client';

import { signup } from '@/app/actions/signup';
import { signupSchema } from '@/app/actions/signup/schema';
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
import { useToast } from '@/components/ui/use-toast';
import { useAction } from '@/hooks/useAction';
import { zodResolver } from '@hookform/resolvers/zod';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import AuthFooter from './AuthFooter';
import SocailSignIns from './SocialSignIns';

const SignUpForm = () => {
  const [isLoading, setIsLoading] = useState(false);
  const signUpForm = useForm<z.infer<typeof signupSchema>>({
    resolver: zodResolver(signupSchema),
    defaultValues: {
      username: '',
      email: '',
      password: '',
    },
  });
  function onSubmit(values: z.infer<typeof signupSchema>) {
    setIsLoading(true);
    execute(values).catch((e) => console.log(e));
  }
  const { toast } = useToast();
  const { execute } = useAction(signup, {
    onError: (error) => {
      toast({
        variant: 'destructive',
        title: error.status,
        description: error.message,
      });
    },
    onSuccess(data) {
      toast({
        title: data,
      });
    },
    onComplete() {
      setIsLoading(false);
    },
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle>Create an account</CardTitle>
        <CardDescription>
          Enter username, email and password below to create
          your account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...signUpForm}>
          <form
            onSubmit={signUpForm.handleSubmit(onSubmit)}
            className='space-y-8'
          >
            <FormField
              control={signUpForm.control}
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
              control={signUpForm.control}
              name='username'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={signUpForm.control}
              name='password'
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input {...field} />
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
                Sign Up
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

export default SignUpForm;
