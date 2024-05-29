import { User } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import UserCard from "./item";

interface UserListProps extends HTMLAttributes<HTMLDivElement> {
  users: User[];
}

const UserList = ({ className, users }: UserListProps) => {
  return (
    <div className={cn("flex flex-col gap-4", className)}>
      {users.map((user) => (
        <UserCard key={user.id} user={user} />
      ))}
    </div>
  );
};

export default UserList;
