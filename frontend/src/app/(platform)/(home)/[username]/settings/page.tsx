import { Skeleton } from "@/components/ui/skeleton";
import { useSession } from "@/context/session_context";

const ProfileSettings = () => {
  const { data, status } = useSession({
    authenticated: true,
  });

  if (status == "loading") {
    return <Skeleton className="w-full h-full" />;
  }

  return <div>ProfileSettings</div>;
};

export default ProfileSettings;
