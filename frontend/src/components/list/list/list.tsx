import { List } from "@/lib/types";
import { HTMLAttributes } from "react";
import ListCard from "./item";

interface ListListProps extends HTMLAttributes<HTMLDivElement> {
  lists: List[];
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const ListList = ({ lists, ...props }: ListListProps) => {
  return (
    <>
      {lists.map((list) => (
        <ListCard {...props} key={list.id} list={list} />
      ))}
    </>
  );
};

export default ListList;
