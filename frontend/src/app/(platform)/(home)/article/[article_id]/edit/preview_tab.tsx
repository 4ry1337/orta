"use client";

import { default_extensions } from "@/lib/default_extension";
import { EditorContent, useEditor } from "@tiptap/react";
import React from "react";
import { FullArticle } from "@/lib/types";

const PreviewTab = ({ article }: { article: FullArticle }) => {
  console.log(JSON.parse(article.content || "{}"));
  const editor = useEditor({
    extensions: [...default_extensions],
    editorProps: {
      attributes: {
        class: "min-h-[50rem] border-0 outline-0 p-2",
      },
    },
    content: JSON.parse(article.content || "{}"),
    editable: false,
  });

  if (!editor) {
    return null;
  }

  return (
    <div className="prose prose-neutral mx-auto prose-sm sm:prose-base md:prose-lg lg:prose-xl dark:prose-invert ">
      <EditorContent editor={editor} />
    </div>
  );
};

export default PreviewTab;
