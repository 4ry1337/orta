import { Checkbox } from "@/components/ui/checkbox";
import { List } from "@/lib/types";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";

interface SelectListListProps extends HTMLAttributes<HTMLDivElement> {
  lists: List[];
  article_id: string;
}

const SelectListList = ({
  lists,
  article_id,
  className,
  ...props
}: SelectListListProps) => {
  return (
    <div {...props} className={cn("flex flex-col gap-3", className)}>
      {lists.map((list) => (
        <div key={list.id} className="flex items-center gap-1">
          <Checkbox id={list.id} />
          <label
            htmlFor={list.id}
            className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            {list.label}
          </label>
        </div>
      ))}
    </div>
  );
};

export default SelectListList;
