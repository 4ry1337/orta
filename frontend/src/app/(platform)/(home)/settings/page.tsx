"use client";

import { get_user } from "@/app/actions/user";
import { useSession } from "@/context/session_context";
import { Skeleton } from "@/components/ui/skeleton";
import useSWR from "swr";
import ProfileForm from "./profile_user";

const SettingsPage = () => {
  const { data: session, status } = useSession({
    authenticated: true,
  });

  const { data: user, mutate } = useSWR(session?.username, get_user);

  if (status == "authenticated" && user) {
    return (
      <div className="mx-auto max-w-xl p-4">
        <ProfileForm user={user} />
      </div>
    );
  }

  return <Skeleton className="w-full h-full" />;
};

export default SettingsPage;
