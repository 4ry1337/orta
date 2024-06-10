"use client";

import { default_extensions } from "@/lib/default_extension";
import { EditorContent, useEditor } from "@tiptap/react";
import Collaboration from "@tiptap/extension-collaboration";
import CollaborationCursor from "@tiptap/extension-collaboration-cursor";
import CharacterCount from "@tiptap/extension-character-count";
import Placeholder from "@tiptap/extension-placeholder";
import MenuBar from "@/components/article/edit/menubar";
import { cn } from "@/lib/utils";
import { HTMLAttributes } from "react";
import { useArticle } from "./page";

interface EditorTabProps extends HTMLAttributes<HTMLDivElement> { }

const EditorTab = ({ className, ...props }: EditorTabProps) => {
  const { provider, session } = useArticle();
  const editor = useEditor({
    extensions: [
      ...default_extensions,
      CharacterCount,
      Placeholder.configure({
        placeholder: "Start writing...",
      }),
      Collaboration.configure({
        document: provider.document,
      }),
      CollaborationCursor.configure({
        provider,
        user: {
          name: session.username,
          color: "#" + Math.floor(Math.random() * 16777215).toString(16),
        },
      }),
    ],
    editorProps: {
      attributes: {
        class: "min-h-[50rem] border-0 outline-0 p-2",
      },
    },
    autofocus: true,
    editable: true,
  });

  if (!editor) {
    return null;
  }

  return (
    <div className={cn("relative", className)}>
      <MenuBar editor={editor} />
      <div className="prose prose-neutral mx-auto prose-sm sm:prose-base md:prose-lg lg:prose-xl dark:prose-invert ">
        <EditorContent editor={editor} />
      </div>
      <div className="flex flew-row items-center justify-start gap-4 p-3">
        <div className="inline-flex flex-row gap-1">
          {editor.storage.characterCount.words()} words
        </div>
      </div>
    </div>
  );
};

export default EditorTab;
