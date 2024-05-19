import { List } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import ListCard from "./item";

interface ListListProps extends HTMLAttributes<HTMLDivElement> {
  lists: List[];
}

const ListList = ({ className, lists }: ListListProps) => {
  return (
    <div className={cn("space-y-4", className)}>
      {lists.map((list) => (
        <ListCard key={list.id} list={list} />
      ))}
    </div>
  );
};

export default ListList;
