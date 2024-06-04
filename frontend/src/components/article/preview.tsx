"use client";

import { default_extensions } from "@/lib/default_extension";
import { cn } from "@/lib/utils";
import { EditorContent, useEditor } from "@tiptap/react";
import { HTMLAttributes } from "react";

interface PreviewProps extends HTMLAttributes<HTMLDivElement> {
  content: string;
}

const Preview = ({ content, ...props }: PreviewProps) => {
  const editor = useEditor({
    extensions: default_extensions,
    content: content,
    editable: false,
  });
  return (
    <div
      className={cn(
        "prose prose-neutral mx-auto prose-sm sm:prose-base md:prose-lg lg:prose-xl dark:prose-invert",
        props.className,
      )}
      {...props}
    >
      <EditorContent editor={editor} />
    </div>
  );
};

export default Preview;
