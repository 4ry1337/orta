import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { List } from "@/lib/types";
import { DisplayDate } from "@/lib/utils";
import { Share1Icon } from "@radix-ui/react-icons";
import Link from "next/link";
import { HTMLAttributes } from "react";

interface ListCardProps extends HTMLAttributes<HTMLDivElement> {
  list: List;
}

const ListCard = ({ list }: ListCardProps) => {
  return (
    <Card>
      <Link href={`/list/${list.slug}`}>
        <CardHeader>
          <CardTitle>{list.label}</CardTitle>
        </CardHeader>
        <CardContent>
          <small>{DisplayDate(list.created_at)}</small>
        </CardContent>
      </Link>
      <CardFooter className="justify-between">
        <div className="inline-flex gap-2">
          <Button variant={"ghost"} size={"icon"}>
            <Share1Icon />
          </Button>
        </div>
      </CardFooter>
    </Card>
  );
};

export default ListCard;
