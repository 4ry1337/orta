import { HTMLAttributes } from 'react';

interface CommentItemProps
  extends HTMLAttributes<HTMLDivElement> {
  comment: Comment;
}

const CommentItem = ({
  comment,
  ...props
}: CommentItemProps) => {
  return <div></div>;
};

export default CommentItem;
