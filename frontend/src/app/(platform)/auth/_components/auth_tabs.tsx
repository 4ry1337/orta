"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useRouter } from "next/navigation";
import { HTMLAttributes } from "react";
import SignUpForm from "./sign_up_form";
import { SignInForm } from "./sign_in_form";

interface AuthTabsProps extends HTMLAttributes<HTMLDivElement> { }

const AuthTabs = (props: AuthTabsProps) => {
  const router = useRouter();
  return (
    <Tabs defaultValue="signin">
      <TabsList className="grid w-full grid-cols-2">
        <TabsTrigger value="signin">Sign In</TabsTrigger>
        <TabsTrigger value="signup">Sign Up</TabsTrigger>
      </TabsList>
      <TabsContent value="signin">
        <SignInForm />
      </TabsContent>
      <TabsContent value="signup">
        <SignUpForm />
      </TabsContent>
    </Tabs>
  );
};

export default AuthTabs;
