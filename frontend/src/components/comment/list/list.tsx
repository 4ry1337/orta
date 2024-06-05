import { HTMLAttributes } from "react";
import CommentCard from "./item";
import { Comment } from "@/lib/types";

interface CommentListProps extends HTMLAttributes<HTMLDivElement> {
  comments: Comment[];
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const ListList = ({ comments, ...props }: CommentListProps) => {
  return (
    <>
      {comments.map((comment) => (
        <CommentCard {...props} key={comment.id} comment={comment} />
      ))}
    </>
  );
};

export default ListList;
