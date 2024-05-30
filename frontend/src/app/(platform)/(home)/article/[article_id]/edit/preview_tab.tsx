"use client";

import { default_extensions } from "@/lib/default_extension";
import { EditorContent, useEditor } from "@tiptap/react";
import Collaboration from "@tiptap/extension-collaboration";
import React, { HTMLAttributes } from "react";
import { cn } from "@/lib/utils";
import { HocuspocusProvider } from "@hocuspocus/provider";

interface PreviewTabProps extends HTMLAttributes<HTMLDivElement> {
  provider: HocuspocusProvider;
}

const PreviewTab = ({ provider, className }: PreviewTabProps) => {
  const editor = useEditor({
    extensions: [
      ...default_extensions,
      Collaboration.configure({
        document: provider.document,
      }),
    ],
    editorProps: {
      attributes: {
        class: "min-h-[50rem] border-0 outline-0 p-2",
      },
    },
    editable: false,
  });

  if (!editor) {
    return null;
  }

  return (
    <div
      className={cn(
        "prose prose-neutral mx-auto prose-sm sm:prose-base md:prose-lg lg:prose-xl dark:prose-invert",
        className,
      )}
    >
      <EditorContent editor={editor} />
    </div>
  );
};

export default PreviewTab;
