import { User } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import UserCard from "./item";

interface UserListProps extends HTMLAttributes<HTMLDivElement> {
  users?: User[];
  badges?: boolean;
}

const UserList = ({ users, badges, ...props }: UserListProps) => {
  if (!users) {
    return null;
  }
  return (
    <>
      {users.map((user) => (
        <UserCard {...props} key={user.id} user={user} />
      ))}
    </>
  );
};

export default UserList;
