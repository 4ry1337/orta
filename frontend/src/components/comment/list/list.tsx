import { HTMLAttributes } from "react";
import CommentCard from "./item";
import { FullComment } from "@/lib/types";

interface CommentListProps extends HTMLAttributes<HTMLDivElement> {
  comments: FullComment[];
  editable?: boolean;
  deletable?: boolean;
  onDelete?: (id: string) => void;
}

const CommentList = ({ comments, ...props }: CommentListProps) => {
  return (
    <>
      {comments.map((comment) => (
        <CommentCard {...props} key={comment.id} comment={comment} />
      ))}
    </>
  );
};

export default CommentList;
