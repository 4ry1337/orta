import { follow, unfollow } from "@/app/actions/user";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { useSession } from "@/context/session_context";
import { FullUser } from "@/lib/types";
import { cn } from "@/lib/utils";
import Link from "next/link";
import { HTMLAttributes, useState } from "react";

interface UserCardProps extends HTMLAttributes<HTMLDivElement> {
  user: FullUser;
}

const UserCard = ({ user, className, badge = false }: UserCardProps) => {
  const { data } = useSession();
  const [followed, setFollowed] = useState(user.followed);

  if (badge) {
    return (
      <div
        className={cn(
          "inline-flex items-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
          className,
        )}
      >
        <Link href={`/${user.username}`}>
          <Avatar className="w-12 h-12">
            <AvatarImage
              src={"http://localhost:5000/api/assets/" + user.image}
              className="object-cover"
              alt="@avatar"
            />
            <AvatarFallback>{user.username.at(0)}</AvatarFallback>
          </Avatar>
        </Link>
        <Button asChild className="px-0" size={"sm"} variant={"link"}>
          <Link href={`/${user.username}`}>
            <div>{user.username}</div>
          </Link>
        </Button>
        {!data || data?.user_id == user.id ? null : followed ? (
          <>
            <span>·</span>
            <Button
              className="px-0"
              size={"sm"}
              variant={"link"}
              onClick={() => {
                unfollow(user.username);
                setFollowed(false);
              }}
            >
              Following
            </Button>
          </>
        ) : (
          <>
            <span>·</span>
            <Button
              className="px-0"
              size={"sm"}
              variant={"link"}
              onClick={() => {
                follow(user.username);
                setFollowed(true);
              }}
            >
              Follow
            </Button>
          </>
        )}
      </div>
    );
  }

  return (
    <div
      className={cn(
        "flex items-center justify-between whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 flex-row hover:bg-accent hover:text-accent-foreground px-4 py-1",
        className,
      )}
    >
      <Link
        href={`/${user.username}`}
        className="flex items-center justify-center"
      >
        <Avatar>
          <AvatarImage
            src={"http://localhost:5000/api/assets/" + user.image}
            className="object-cover"
            alt="@avatar"
          />
          <AvatarFallback>{user.username.at(0)}</AvatarFallback>
        </Avatar>
        <div className="ml-2 grow spacy-y-4">
          <h4>{user.username}</h4>
        </div>
      </Link>
      {!data || data?.user_id == user.id ? null : followed ? (
        <Button
          onClick={() => {
            unfollow(user.username);
            setFollowed(false);
          }}
        >
          Unfollow
        </Button>
      ) : (
        <Button
          onClick={() => {
            follow(user.username);
            setFollowed(true);
          }}
        >
          Follow
        </Button>
      )}
    </div>
  );
};

export default UserCard;
