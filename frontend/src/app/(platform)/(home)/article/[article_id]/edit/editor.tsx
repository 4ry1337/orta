"use client";

import { TiptapCollabProvider } from "@hocuspocus/provider";
import {
  useEditor,
  EditorContent,
  BubbleMenu,
  FloatingMenu,
  Editor,
} from "@tiptap/react";
import CharacterCount from "@tiptap/extension-character-count";
import Document from "@tiptap/extension-document";
import Paragraph from "@tiptap/extension-paragraph";
import Text from "@tiptap/extension-text";
import History from "@tiptap/extension-history";
import Heading from "@tiptap/extension-heading";
import Blockquote from "@tiptap/extension-blockquote";
import OrderedList from "@tiptap/extension-ordered-list";
import ListItem from "@tiptap/extension-list-item";
import BulletList from "@tiptap/extension-bullet-list";
import Highlight from "@tiptap/extension-highlight";
import CodeBlock from "@tiptap/extension-code-block";
import Placeholder from "@tiptap/extension-placeholder";
import Youtube from "@tiptap/extension-youtube";
import Code from "@tiptap/extension-code";
import Bold from "@tiptap/extension-bold";
import Italic from "@tiptap/extension-italic";
import Strike from "@tiptap/extension-strike";
import Subscript from "@tiptap/extension-subscript";
import Link from "@tiptap/extension-link";
import Superscript from "@tiptap/extension-superscript";
import Underline from "@tiptap/extension-underline";
import Collaboration from "@tiptap/extension-collaboration";
import * as Y from "yjs";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import {
  Undo,
  Redo,
  UnderlineIcon,
  ItalicIcon,
  BoldIcon,
  TextIcon,
  Heading1Icon,
  Heading2Icon,
  Heading3Icon,
  Heading4Icon,
  Heading5Icon,
  Heading6Icon,
  ListIcon,
  ListOrderedIcon,
  QuoteIcon,
  CodeIcon,
  YoutubeIcon,
  PlusIcon,
} from "lucide-react";
import { Toggle } from "@/components/ui/toggle";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { useEffect, useState } from "react";
import { Article } from "@/lib/types";

const YoutubeButton = ({ editor }: { editor: Editor }) => {
  const [link, setlink] = useState("");
  return (
    <Dialog>
      <DialogTrigger>
        <YoutubeIcon />
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>Add Youtube Video</DialogTitle>
        <DialogDescription>
          <Input
            onChange={(val) => setlink(val.target.value)}
            placeholder="https://youtube.com/..."
          />
        </DialogDescription>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="secondary">
              Close
            </Button>
          </DialogClose>
          <DialogClose asChild>
            <Button
              onClick={() => {
                console.log(link);
                editor.commands.setYoutubeVideo({
                  src: link,
                });
              }}
            >
              Add
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

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
            <Heading1Icon className="icon h1" />
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

// <div className="inline-flex flex-row items-center justify-start gap-1">
//   <Button
//     onClick={() => editor.chain().focus().undo().run()}
//     disabled={!editor.can().undo()}
//     size={"icon"}
//     variant={"ghost"}
//   >
//     <Undo />
//   </Button>
//   <Button
//     size={"icon"}
//     variant={"ghost"}
//     disabled={!editor.can().redo()}
//     onClick={() => editor.chain().focus().redo().run()}
//   >
//     <Redo />
//   </Button>
// </div>

const MenuBar = ({ editor }: { editor: Editor }) => {
  return (
    <div className="rounded-t-xl bg-card border-b sticky z-10 top-0">
      <ScrollArea>
        <div className="flex flew-row items-center justify-center gap-4 p-3">
          <div className="inline-flex flex-row items-center justify-start gap-1">
            <FormatSelect editor={editor} />
          </div>
          <div className="inline-flex flex-row items-center justify-start gap-1">
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
            </Toggle>
          </div>
          <div className="inline-flex flex-row items-center justify-start gap-1">
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
          <div className="inline-flex flex-row items-center justify-start gap-1">
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
          <div className="inline-flex flex-row items-center justify-start gap-1">
            <YoutubeButton editor={editor} />
          </div>
        </div>
        <ScrollBar orientation="horizontal" />
      </ScrollArea>
    </div>
  );
};

const ArticleEditor = ({ article }: { article: Article }) => {
  const doc = new Y.Doc();

  const provider = new TiptapCollabProvider({
    name: article.id, // Unique document identifier for syncing. This is your document name.
    appId: "7j9y6m10", // Your Cloud Dashboard AppID or `baseURL` for on-premises
    token: sessionStorage.getItem("session"), // Your JWT token
    document: doc,
    onOpen() {
      console.log("WebSocket connection opened.");
    },
    onConnect() {
      console.log("Connected to the server.");
    },
    onDisconnect() {
      console.log("Disconnected to the server.");
    },
  });

  provider.on("synced", () => {
    console.log("Document synced.");
  });

  const editor = useEditor({
    extensions: [
      // History,
      CharacterCount,
      Document,
      Paragraph,
      Text,
      CodeBlock,
      Blockquote,
      BulletList,
      OrderedList,
      ListItem,
      Heading,
      Bold,
      Italic,
      Code,
      Strike,
      Subscript,
      Superscript,
      Link,
      Underline,
      Placeholder.configure({
        placeholder: "Start writing...",
      }),
      Youtube.configure({
        allowFullscreen: true,
        nocookie: true,
        height: undefined,
        width: undefined,
        HTMLAttributes: {
          class: "w-full aspect-video",
        },
      }),
      // Collaboration.configure({
      //   document: doc,
      // }),
    ],
    editorProps: {
      attributes: {
        class:
          "prose prose-neutral mx-auto prose-sm sm:prose-base md:prose-lg lg:prose-xl dark:prose-invert min-h-[50rem] border-0 outline-0 p-2",
      },
    },
    autofocus: true,
    editable: true,
  });

  if (!editor) {
    return null;
  }

  return (
    <>
      <div className="rounded-xl border bg-card text-card-foreground shadow">
        {editor.isEditable && <MenuBar editor={editor} />}
        <EditorContent editor={editor} />
        <div className="inline-flex flew-row items-center justify-start gap-4 p-3">
          <div className="inline-flex flex-row items-center justify-start gap-1">
            <div className="">
              {editor.storage.characterCount.words()} words
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default ArticleEditor;
