import React from "react";
import { cn } from "@/lib/utils";
import SortableList, { SortableItem } from "react-easy-sort";
import { Badge } from "../ui/badge";

export type TagListProps = {
  tags: string[];
  draggable?: boolean;
  onTagClick: (tag: string) => void;
  onSortEnd: (oldIndex: number, newIndex: number) => void;
};

const DropTarget: React.FC = () => {
  return <Badge className={cn("h-full rounded-md bg-secondary/50")} />;
};

export const TagList: React.FC<TagListProps> = ({
  tags,
  draggable = false,
  onTagClick,
  onSortEnd,
}) => {
  const [draggedTag, setDraggedTag] = React.useState<string | null>(null);

  const handleMouseDown = (tag: string) => {
    setDraggedTag(tag);
  };

  const handleMouseUp = () => {
    setDraggedTag(null);
  };

  return (
    <div
      className={cn("rounded-md max-w-[450px] flex flex-wrap gap-2")}
    >
      {draggable ? (
        <SortableList
          onSortEnd={onSortEnd}
          className="flex flex-wrap gap-2 list"
          dropTarget={<DropTarget />}
        >
          {tags.map((tag) => (
            <SortableItem key={tag}>
              <div
                onMouseDown={() => handleMouseDown(tag)}
                onMouseLeave={handleMouseUp}
                className={cn(
                  {
                    "border border-solid border-primary rounded-md":
                      draggedTag === tag,
                  },
                  "transition-all duration-200 ease-in-out"
                )}
              >
                <Badge
                  draggable={draggable}
                  className="cursor-pointer"
                  onClick={() => onTagClick?.(tag)}
                >
                  {tag}
                </Badge>
              </div>
            </SortableItem>
          ))}
        </SortableList>
      ) : (
        tags.map((tag) =>
          <Badge
            key={tag}
            onClick={() => onTagClick?.(tag)}
          >
            {tag}
          </Badge>
        )
      )}
    </div>
  );
};
