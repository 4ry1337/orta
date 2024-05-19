"use client";

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
import { SignInFormSchema } from "@/lib/definitions";
import { signin } from "@/app/actions/auth";

export function SignInForm() {
  const [pending, startTransition] = useTransition();

  const SignInForm = useForm<z.infer<typeof SignInFormSchema>>({
    resolver: zodResolver(SignInFormSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof SignInFormSchema>) => {
    startTransition(async () => {
      await signin(values);
    });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Login</CardTitle>
        <CardDescription>
          Enter email and password below to access your account
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...SignInForm}>
          <form
            onSubmit={SignInForm.handleSubmit(onSubmit)}
            className="space-y-8"
          >
            <FormField
              control={SignInForm.control}
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
              control={SignInForm.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input type="password" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <div>
              <Button
                aria-disabled={
                  pending ||
                  !SignInForm.formState.isValid ||
                  SignInForm.formState.isLoading
                }
                disabled={
                  pending ||
                  !SignInForm.formState.isValid ||
                  SignInForm.formState.isLoading
                }
                className="w-full"
                type="submit"
              >
                Sign In
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
}
