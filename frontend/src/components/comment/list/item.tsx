import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Card,
  CardTitle,
  CardFooter,
  CardContent,
  CardHeader,
} from "@/components/ui/card";
import { FullComment } from "@/lib/types";
import { DisplayDate } from "@/lib/utils";
import { HTMLAttributes } from "react";

interface CommentCardProps extends HTMLAttributes<HTMLDivElement> {
  comment: FullComment;
  // editable?: boolean;
  // deletable?: boolean;
  // onDelete?: (id: string) => void;
}

const CommentCard = ({ comment, ...props }: CommentCardProps) => {
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          <div className="inline-flex items-center gap-2 whitespace-nowrap rounded-md text-sm font-medium">
            <Avatar>
              <AvatarImage
                src={
                  comment.image &&
                  "http://localhost:5000/api/assets/" + comment.image
                }
                className="object-cover"
                alt="@avatar"
              />
              <AvatarFallback>{comment.username.at(0)}</AvatarFallback>
            </Avatar>
            <span className="ml-2">{comment.username}</span>
          </div>
        </CardTitle>
      </CardHeader>
      <CardContent>{comment.content}</CardContent>
      <CardFooter className="text-sm text-muted-foreground">
        {DisplayDate(comment.created_at)} {comment.updated_at && "(edited)"}
      </CardFooter>
    </Card>
  );
};

export default CommentCard;
