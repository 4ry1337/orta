import { signout } from "@/app/actions/auth";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Separator } from "@/components/ui/separator";
import { Session } from "@/lib/types";
import { cn } from "@/lib/utils";
import { ActivityLogIcon, ExitIcon, GearIcon } from "@radix-ui/react-icons";
import Link from "next/link";
import { HTMLAttributes, useTransition } from "react";

interface UserButtonProps extends HTMLAttributes<HTMLDivElement> {
  user: Session;
}

const UserButton = ({ className, user }: UserButtonProps) => {
  const [pending, startTransition] = useTransition();
  const onSubmit = async () => {
    startTransition(async () => {
      await signout();
    });
  };
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button
          variant={"outline"}
          className={cn(
            "scroll-m-20 text-2xl font-semibold tracking-tight h-14 rounded-full w-14 p-2 xl:w-auto xl:justify-start",
            className,
          )}
        >
          <Avatar>
            <AvatarImage
              src={
                user.image && "http://localhost:5000/api/assets/" + user.image
              }
              className="object-cover"
              alt="@avatar"
            />
            <AvatarFallback>{user.username.at(0)}</AvatarFallback>
          </Avatar>
          <span className="ml-2 hidden xl:block">{user.username}</span>
        </Button>
      </PopoverTrigger>
      <PopoverContent align="start" className="w-60 rounded-full">
        <div className="flex flex-col gap-3">
          <Button
            onClick={() => onSubmit()}
            className="rounded-full"
            variant={"destructive"}
          >
            <ExitIcon className="mr-2" />
            Sign Out
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  );
};

// <Button variant={"secondary"} className="justify-start" asChild>
//   <Link href={"/activity"} prefetch={false} className="justify-start">
//     <ActivityLogIcon className="mr-2" />
//     <div>Activity Log</div>
//   </Link>
// </Button>
// <Button variant={"secondary"} className="justify-start" asChild>
//   <Link href={"/settings"} prefetch={false}>
//     <GearIcon className="mr-2" />
//     Settigns
//   </Link>
// </Button>
// <Separator />
export default UserButton;
