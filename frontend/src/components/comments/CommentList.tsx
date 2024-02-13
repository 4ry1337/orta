import { Comment } from '@/types';
import { HTMLAttributes } from 'react';
import { ScrollArea } from '../ui/scroll-area';
import CommentItem from './CommentItem';

async function getComments(id: number): Promise<Comment[]> {
  //   return fetch(`http://localhost:5000/api/article/comments`, {
  //     method: 'POST',
  //     headers: {
  //       'Content-Type': 'application/json',
  //     },
  //     body: JSON.stringify({
  //       article: [id],
  //     }),
  //   }).then(async (res) => {
  //     const response = await res.json();
  //     return response;
  //   });
  return [];
}

interface CommentListProps
  extends HTMLAttributes<HTMLDivElement> {
  article_id: number;
}

const CommentList = async ({
  article_id,
  ...props
}: CommentListProps) => {
  const comments = await getComments(article_id);
  return (
    <ScrollArea>
      <div className='flex flex-col'>
        {comments.map((comment) => (
          <CommentItem
            key={comment.id}
            comment={comment}
          />
        ))}
      </div>
    </ScrollArea>
  );
};

export default CommentList;
