"use client";

import LogoIcon from "@/components/logo";
import AuthTabs from "./_components/auth_tabs";
import { useSession } from "@/context/session_context";
import { Skeleton } from "@/components/ui/skeleton";
import useSWR from "swr";
import { verify } from "@/app/actions/auth";

const Auth = ({
  searchParams,
}: {
  searchParams: {
    token: string | null;
  };
}) => {
  const { status } = useSession({
    authenticated: false,
  });

  useSWR(searchParams.token, verify);

  if (status == "loading") {
    return <Skeleton className="h-screen w-full" />;
  }

  return (
    <div className="h-screen grid lg:grid-cols-[1fr,45vw]">
      <div className="hidden flex-col justify-between bg-muted p-10 lg:flex">
        <div className="flex items-center text-lg font-medium">
          <LogoIcon />
          <span className="ml-2">Orta</span>
        </div>
        <blockquote className="space-y-2">
          <p className="text-lg">
            &ldquo;A platform for creative professionals to create articles and
            effectively interact with audiences online.&rdquo;
          </p>
          <footer className="text-sm">Orta Developers</footer>
        </blockquote>
      </div>
      <div className="py-10 lg:p-10">
        <div className="mx-auto max-w-96">
          <AuthTabs />
        </div>
      </div>
    </div>
  );
};

export default Auth;
