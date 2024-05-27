"use client";

import { LexicalComposer } from "@lexical/react/LexicalComposer";
import { HTMLAttributes, useState } from "react";
import { toast } from "sonner";
import editor_theme from "./editor_theme";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { ListPlugin } from "@lexical/react/LexicalListPlugin";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import MarkdownPlugin from "./plugin/markdown/shortcut_plugin";
import Nodes from "./nodes";
import { AutoFocusPlugin } from "./plugin/auto_focus_plugin";
import ToolbarPlugin from "./plugin/toolbar";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { LinkPlugin } from "@lexical/react/LexicalLinkPlugin";
import FloatingLinkEditorPlugin from "./plugin/floating_link_editor";

interface EditorProps extends HTMLAttributes<HTMLDivElement> { }

const Editor = ({ className, ...props }: EditorProps) => {
  const initialConfig = {
    namespace: "Article",
    onError: (error: Error) => {
      toast.error(error.message);
    },
    editorState: null,
    nodes: Nodes,
  };

  const [isLinkEditMode, setIsLinkEditMode] = useState<boolean>(false);

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <div className="rounded-xl border bg-card text-card-foreground shadow">
        <div className="sticky z-10 top-0">
          <ScrollArea className="p-2 pb-4">
            <ToolbarPlugin
              className="p-1"
              setIsLinkEditMode={setIsLinkEditMode}
            />
            <ScrollBar orientation="horizontal" />
          </ScrollArea>
        </div>
        <div className="relative">
          <RichTextPlugin
            contentEditable={
              <ContentEditable
                className={"border-0 min-h-[50rem] relative outline-0 p-2"}
              />
            }
            placeholder={
              <p className="absolute top-2 left-2 overflow-hidden text-ellipsis">
                Enter some text...
              </p>
            }
            ErrorBoundary={LexicalErrorBoundary}
          />
          <ListPlugin />
          <LinkPlugin />
          <AutoFocusPlugin />
          <MarkdownPlugin />
          <HistoryPlugin />
        </div>
      </div>
    </LexicalComposer>
  );
};

// <FloatingTextFormatToolbarPlugin
//   anchorElem={floatingAnchorElem}
//   setIsLinkEditMode={setIsLinkEditMode}
// />
// <CodeActionMenuPlugin anchorElem={floatingAnchorElem} />
export default Editor;
