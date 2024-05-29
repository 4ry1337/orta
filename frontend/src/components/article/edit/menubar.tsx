import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import {
  UnderlineIcon,
  ItalicIcon,
  BoldIcon,
  ListIcon,
  ListOrderedIcon,
  QuoteIcon,
  CodeIcon,
  Redo,
  Undo,
} from "lucide-react";
import { Toggle } from "@/components/ui/toggle";
import FormatSelect from "@/components/article/edit/format_select";
import YoutubeButton from "@/components/article/edit/youtube_button";
import { Editor } from "@tiptap/react";

const MenuBar = ({ editor }: { editor: Editor }) => {
  return (
    <div className="bg-card border-b sticky z-10 top-0">
      <ScrollArea>
        <div className="flex flew-row items-center justify-start gap-4 p-3">
          <div className="inline-flex flex-row gap-1">
            <Button
              onClick={() => editor.chain().focus().undo().run()}
              disabled={!editor.can().undo()}
              size={"icon"}
              variant={"ghost"}
            >
              <Undo />
            </Button>
            <Button
              size={"icon"}
              variant={"ghost"}
              disabled={!editor.can().redo()}
              onClick={() => editor.chain().focus().redo().run()}
            >
              <Redo />
            </Button>
          </div>
          <div className="inline-flex flex-row gap-1">
            <FormatSelect editor={editor} />
          </div>
          <div className="inline-flex flex-row gap-1">
            <Toggle
              pressed={editor.isActive("bold")}
              onClick={() => editor.chain().focus().toggleBold().run()}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <BoldIcon />
              </Button>
            </Toggle>
            <Toggle
              onClick={() => editor.chain().focus().toggleItalic().run()}
              pressed={editor.isActive("italic")}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <ItalicIcon />
              </Button>
            </Toggle>
            <Toggle
              onClick={() => editor.chain().focus().toggleUnderline().run()}
              pressed={editor.isActive("underlint")}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <UnderlineIcon />
              </Button>
            </Toggle>{" "}
          </div>
          <div className="inline-flex flex-row gap-1">
            <Toggle
              pressed={editor.isActive("blockquote")}
              onClick={() => editor.chain().focus().toggleBlockquote().run()}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <QuoteIcon />
              </Button>
            </Toggle>
            <Toggle
              onClick={() => editor.chain().focus().toggleCodeBlock().run()}
              pressed={editor.isActive("codeBlock")}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <CodeIcon />
              </Button>
            </Toggle>
          </div>
          <div className="inline-flex flex-row gap-1">
            <Toggle
              pressed={editor.isActive("bulletList")}
              onClick={() => editor.chain().focus().toggleBulletList().run()}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <ListOrderedIcon />
              </Button>
            </Toggle>
            <Toggle
              pressed={editor.isActive("orderedList")}
              onClick={() => editor.chain().focus().toggleOrderedList().run()}
              asChild
            >
              <Button variant={"secondary"} size={"icon"}>
                <ListIcon />
              </Button>
            </Toggle>
          </div>
          <div className="inline-flex flex-row gap-1">
            <YoutubeButton editor={editor} />
          </div>
        </div>
        <ScrollBar orientation="horizontal" />
      </ScrollArea>
    </div>
  );
};

export default MenuBar;
