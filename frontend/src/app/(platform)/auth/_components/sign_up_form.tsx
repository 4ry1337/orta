"use client";

import { SignUpFormSchema } from "@/app/lib/definitions";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import AuthFooter from "./auth_footer";
import { useTransition } from "react";
import { signup } from "@/app/actions/auth";
// 478rhdfigf857#
const SignUpForm = () => {
  const [, startTransition] = useTransition();

  const SignUpForm = useForm<z.infer<typeof SignUpFormSchema>>({
    resolver: zodResolver(SignUpFormSchema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
    },
    mode: "onChange",
  });

  const onSubmit = (values: z.infer<typeof SignUpFormSchema>) => {
    startTransition(async () => {
      await signup(values);
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Create an account</CardTitle>
        <CardDescription>
          Enter username, email and password below to create your account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...SignUpForm}>
          <form
            onSubmit={SignUpForm.handleSubmit(onSubmit)}
            className="space-y-8"
          >
            <FormField
              control={SignUpForm.control}
              name="email"
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
              control={SignUpForm.control}
              name="username"
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
              control={SignUpForm.control}
              name="password"
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
                aria-disabled={
                  !SignUpForm.formState.isValid ||
                  SignUpForm.formState.isLoading
                }
                disabled={
                  !SignUpForm.formState.isValid ||
                  SignUpForm.formState.isLoading
                }
                className="w-full"
                type="submit"
              >
                Sign Up
              </Button>
            </div>
          </form>
        </Form>
      </CardContent>
      <CardFooter>
        <AuthFooter />
      </CardFooter>
    </Card>
  );
};

// <div className='mt-6'>
//   <div className='relative'>
//     <div className='absolute inset-0 flex items-center'>
//       <div className='w-full border-t border-muted-foreground' />
//     </div>
//     <div className='relative flex justify-center text-center text-sm text-muted-foreground'>
//       <span className='bg-background px-2'>
//         Or continue with
//       </span>
//     </div>
//   </div>
//   <SocailSignIns className={'mt-6'} />
// </div>

export default SignUpForm;
