import { FullUser } from "@/lib/types";
import { HTMLAttributes } from "react";
import UserCard from "./item";

interface UserListProps extends HTMLAttributes<HTMLDivElement> {
  users?: FullUser[];
  badge?: boolean;
}

const UserList = ({ users, ...props }: UserListProps) => {
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
