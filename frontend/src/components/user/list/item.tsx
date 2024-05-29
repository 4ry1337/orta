import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { User } from "@/lib/types";
import Link from "next/link";
import { HTMLAttributes } from "react";

interface UserCardProps extends HTMLAttributes<HTMLDivElement> {
  user: User;
}

const UserCard = ({ user }: UserCardProps) => {
  return (
    <Card>
      <Link href={`/${user.username}`}>
        <CardHeader>
          <CardTitle>{user.username}</CardTitle>
        </CardHeader>
      </Link>
    </Card>
  );
};

export default UserCard;
