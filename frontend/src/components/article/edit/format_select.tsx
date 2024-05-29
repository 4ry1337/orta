import {
  Select,
  SelectItem,
  SelectValue,
  SelectContent,
  SelectTrigger,
} from "@/components/ui/select";
import { Editor } from "@tiptap/react";
import {
  Heading1Icon,
  Heading2Icon,
  Heading3Icon,
  Heading4Icon,
  Heading5Icon,
  Heading6Icon,
  TextIcon,
} from "lucide-react";

const FormatSelect = ({ editor }: { editor: Editor }) => {
  return (
    <Select
      value={
        editor.isActive("heading", { level: 1 })
          ? "h1"
          : editor.isActive("heading", { level: 2 })
            ? "h2"
            : editor.isActive("heading", { level: 3 })
              ? "h3"
              : editor.isActive("heading", { level: 4 })
                ? "h4"
                : editor.isActive("heading", { level: 5 })
                  ? "h5"
                  : editor.isActive("heading", { level: 6 })
                    ? "h6"
                    : editor.isActive("paragraph")
                      ? "paragraph"
                      : ""
      }
      onValueChange={(value) => {
        if (value == "paragraph") editor.chain().focus().setParagraph().run();
        if (value == "h1")
          editor.chain().focus().setHeading({ level: 1 }).run();
        if (value == "h2")
          editor.chain().focus().setHeading({ level: 2 }).run();
        if (value == "h3")
          editor.chain().focus().setHeading({ level: 3 }).run();
        if (value == "h4")
          editor.chain().focus().setHeading({ level: 4 }).run();
        if (value == "h5")
          editor.chain().focus().setHeading({ level: 5 }).run();
        if (value == "h6")
          editor.chain().focus().setHeading({ level: 6 }).run();
      }}
    >
      <SelectTrigger className="w-48">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="paragraph">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <TextIcon />
            <span className="text-nowrap">Normal</span>
          </div>
        </SelectItem>
        <SelectItem value="h1">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading1Icon />
            <span className="text-nowrap">Heading 1</span>
          </div>
        </SelectItem>
        <SelectItem value="h2">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading2Icon />
            <span className="text-nowrap">Heading 2</span>
          </div>
        </SelectItem>
        <SelectItem value="h3">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading3Icon />
            <span className="text-nowrap">Heading 3</span>
          </div>
        </SelectItem>
        <SelectItem value="h4">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading4Icon />
            <span className="text-nowrap">Heading 4</span>
          </div>
        </SelectItem>
        <SelectItem value="h5">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading5Icon />
            <span className="text-nowrap">Heading 5</span>
          </div>
        </SelectItem>
        <SelectItem value="h6">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading6Icon />
            <span className="text-nowrap">Heading 6</span>
          </div>
        </SelectItem>
      </SelectContent>
    </Select>
  );
};
export default FormatSelect;
