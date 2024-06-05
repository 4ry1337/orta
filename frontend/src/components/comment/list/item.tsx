import { Comment } from "@/lib/types";
import { HTMLAttributes } from "react";

interface CommentCardProps extends HTMLAttributes<HTMLDivElement> {
  comment: Comment;
  // editable?: boolean;
  // deletable?: boolean;
  // onDelete?: (id: string) => void;
}

const CommentCard = ({ comment, ...props }: CommentCardProps) => {
  return <div>CommentCard</div>;
};

export default CommentCard;
