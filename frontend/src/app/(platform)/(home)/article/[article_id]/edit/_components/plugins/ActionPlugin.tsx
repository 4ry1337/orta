import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { exportFile, importFile } from "@lexical/file";
import { useCollaborationContext } from "@lexical/react/LexicalCollaborationContext";
import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { mergeRegister } from "@lexical/utils";
import { CONNECTED_COMMAND } from "@lexical/yjs";
import type { LexicalEditor } from "lexical";
import { $getRoot, $isParagraphNode, COMMAND_PRIORITY_EDITOR } from "lexical";
import { Download, Mic, Upload } from "lucide-react";
import { HTMLAttributes, useEffect, useState } from "react";
import { SPEECH_TO_TEXT_COMMAND } from "./SpeechToTextPlugin";

async function sendEditorState(editor: LexicalEditor): Promise<void> {
  const stringifiedEditorState = JSON.stringify(editor.getEditorState());
  try {
    //TODO: change url
    await fetch("http://localhost:1235/setEditorState", {
      body: stringifiedEditorState,
      headers: {
        Accept: "application/json",
        "Content-type": "application/json",
      },
      method: "POST",
    });
  } catch {
    // NO-OP
  }
}

async function validateEditorState(editor: LexicalEditor): Promise<void> {
  const stringifiedEditorState = JSON.stringify(editor.getEditorState());
  let response = null;
  try {
    //TODO: change url
    response = await fetch("http://localhost:1235/validateEditorState", {
      body: stringifiedEditorState,
      headers: {
        Accept: "application/json",
        "Content-type": "application/json",
      },
      method: "POST",
    });
  } catch {
    // NO-OP
  }
  if (response !== null && response.status === 403) {
    throw new Error(
      "Editor state validation failed! Server did not accept changes.",
    );
  }
}

interface ActionPluginProps extends HTMLAttributes<HTMLDivElement> { }

const ActionsPlugin = ({ className, ...props }: ActionPluginProps) => {
  const [editor] = useLexicalComposerContext();
  const [isEditable, setIsEditable] = useState(() => editor.isEditable());
  const [isSpeechToText, setIsSpeechToText] = useState(false);
  const [connected, setConnected] = useState(false);
  const [isEditorEmpty, setIsEditorEmpty] = useState(true);
  // const { isCollabActive } = useCollaborationContext();

  useEffect(() => {
    return mergeRegister(
      editor.registerEditableListener((editable) => {
        setIsEditable(editable);
      }),
      editor.registerCommand<boolean>(
        CONNECTED_COMMAND,
        (payload) => {
          const isConnected = payload;
          setConnected(isConnected);
          return false;
        },
        COMMAND_PRIORITY_EDITOR,
      ),
    );
  }, [editor]);

  useEffect(() => {
    return editor.registerUpdateListener(
      ({ dirtyElements, prevEditorState, tags }) => {
        // If we are in read only mode, send the editor state
        // to server and ask for validation if possible.
        if (
          !isEditable &&
          dirtyElements.size > 0 &&
          !tags.has("historic") &&
          !tags.has("collaboration")
        ) {
          validateEditorState(editor);
        }
        editor.getEditorState().read(() => {
          const root = $getRoot();
          const children = root.getChildren();

          if (children.length > 1) {
            setIsEditorEmpty(false);
          } else {
            if ($isParagraphNode(children[0])) {
              const paragraphChildren = children[0].getChildren();
              setIsEditorEmpty(paragraphChildren.length === 0);
            } else {
              setIsEditorEmpty(false);
            }
          }
        });
      },
    );
  }, [editor, isEditable]);

  return (
    <div className={cn(className, "flex gap-2")}>
      <Button
        size={"icon"}
        variant={"ghost"}
        onClick={() => {
          editor.dispatchCommand(SPEECH_TO_TEXT_COMMAND, !isSpeechToText);
          setIsSpeechToText(!isSpeechToText);
        }}
        className={cn(isSpeechToText ? "active" : "")}
        title="Speech To Text"
        aria-label={`${isSpeechToText ? "Enable" : "Disable"} speech to text`}
      >
        <Mic />
      </Button>
      <Button
        variant={"ghost"}
        size={"icon"}
        className="action-button import"
        onClick={() => importFile(editor)}
        title="Import"
        aria-label="Import editor state from JSON"
      >
        <Upload />
      </Button>
      <Button
        variant={"ghost"}
        size={"icon"}
        className="action-button export"
        onClick={() =>
          exportFile(editor, {
            fileName: `Playground ${new Date().toISOString()}`,
            source: "Playground",
          })
        }
        title="Export"
        aria-label="Export editor state to JSON"
      >
        <Download />
      </Button>
    </div>
  );
};

export default ActionsPlugin;
