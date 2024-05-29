"use client";

import { default_extensions } from "@/lib/default_extension";
import { EditorContent, useEditor } from "@tiptap/react";
import { HocuspocusProvider } from "@hocuspocus/provider";
import Collaboration from "@tiptap/extension-collaboration";
import CollaborationCursor from "@tiptap/extension-collaboration-cursor";
import CharacterCount from "@tiptap/extension-character-count";
import Placeholder from "@tiptap/extension-placeholder";
import MenuBar from "@/components/article/edit/menubar";

const EditorTab = ({
  username,
  article_id,
}: {
  article_id: string;
  username: string;
}) => {
  const provider = new HocuspocusProvider({
    url: "ws://127.0.0.1:6565",
    name: article_id,
    parameters: {
      token: sessionStorage.getItem("session"),
    },
  });

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
          name: username,
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
    <div>
      <div>
        <MenuBar editor={editor} />
      </div>
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
