import { useToast } from '@/components/ui/use-toast';
import { CAN_USE_DOM, cn } from '@/lib/utils';
import { CheckListPlugin } from '@lexical/react/LexicalCheckListPlugin';
import { ClearEditorPlugin } from '@lexical/react/LexicalClearEditorPlugin';
import { LexicalComposer } from '@lexical/react/LexicalComposer';
import { ContentEditable } from '@lexical/react/LexicalContentEditable';
import LexicalErrorBoundary from '@lexical/react/LexicalErrorBoundary';
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin';
import { HorizontalRulePlugin } from '@lexical/react/LexicalHorizontalRulePlugin';
import { ListPlugin } from '@lexical/react/LexicalListPlugin';
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin';
import { HTMLAttributes, useEffect, useState } from 'react';
import editor_theme from './EditorTheme';
import nodes from './nodes/nodes';
import ActionsPlugin from './plugins/ActionPlugin';
import AutoFocusPlugin from './plugins/AutoFocusPlugin';
import DraggableBlockPlugin from './plugins/DraggableBlockPlugin';
import KeywordsPlugin from './plugins/KeywordsPlugin';
import LinkPlugin from './plugins/LinkPlugin';
import ListMaxIndentLevelPlugin from './plugins/ListMaxIndentLevelPlugin';
import MarkdownShortcutPlugin from './plugins/MarkdownShortcutPlugin';
import ToolbarPlugin from './plugins/ToolbarPlugin';

interface EditorProps
  extends HTMLAttributes<HTMLDivElement> {}

const Editor = ({ className, ...props }: EditorProps) => {
  const { toast } = useToast();

  const initialConfig = {
    // NOTE: This is critical for collaboration plugin to set editor state to null. It
    // would indicate that the editor should not try to set any default state
    // (not even empty one), and let collaboration plugin do it instead
    editorState: null,
    namespace: 'Article',
    nodes: [...nodes],
    onError: (error: Error) => {
      toast({
        variant: 'destructive',
        title: error.message,
      });
    },
    theme: { editor_theme },
  };

  const text = 'Enter text';
  const placeholder = (
    <div className='absolute left-8 top-2.5'>{text}</div>
  );

  const [floatingAnchorElem, setFloatingAnchorElem] =
    useState<HTMLDivElement | null>(null);

  const [isSmallWidthViewport, setIsSmallWidthViewport] =
    useState<boolean>(false);

  const onRef = (_floatingAnchorElem: HTMLDivElement) => {
    if (_floatingAnchorElem !== null) {
      setFloatingAnchorElem(_floatingAnchorElem);
    }
  };

  const [isLinkEditMode, setIsLinkEditMode] =
    useState<boolean>(false);

  useEffect(() => {
    const updateViewPortWidth = () => {
      const isNextSmallWidthViewport =
        CAN_USE_DOM &&
        window.matchMedia('(max-width: 1025px)').matches;

      if (
        isNextSmallWidthViewport !== isSmallWidthViewport
      ) {
        setIsSmallWidthViewport(isNextSmallWidthViewport);
      }
    };
    updateViewPortWidth();
    window.addEventListener('resize', updateViewPortWidth);

    return () => {
      window.removeEventListener(
        'resize',
        updateViewPortWidth
      );
    };
  }, [isSmallWidthViewport]);

  return (
    <div
      className={cn(className, 'relative flex flex-col')}
      {...props}
    >
      <LexicalComposer initialConfig={initialConfig}>
        <ToolbarPlugin
          className='mb-3'
          setIsLinkEditMode={setIsLinkEditMode}
        />
        <div
          className='relative flex min-h-0 shrink grow flex-col'
          ref={onRef}
        >
          <RichTextPlugin
            contentEditable={
              <ContentEditable className='grow overflow-y-scroll rounded-md border border-input bg-background px-8 py-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2' />
            }
            placeholder={placeholder}
            ErrorBoundary={LexicalErrorBoundary}
          />
          <ListPlugin />
          <CheckListPlugin />
          <ListMaxIndentLevelPlugin maxDepth={7} />
          <LinkPlugin />
          <KeywordsPlugin />
          <HistoryPlugin />
          <AutoFocusPlugin />
          <HorizontalRulePlugin />
          <ClearEditorPlugin />
          <CheckListPlugin />
          <MarkdownShortcutPlugin />
          {floatingAnchorElem && !isSmallWidthViewport && (
            <>
              <DraggableBlockPlugin
                anchorElem={floatingAnchorElem}
              />
              {/* <CodeActionMenuPlugin
              anchorElem={floatingAnchorElem}
            />
            <FloatingLinkEditorPlugin
              anchorElem={floatingAnchorElem}
              isLinkEditMode={isLinkEditMode}
              setIsLinkEditMode={setIsLinkEditMode}
            />
            <TableCellActionMenuPlugin
              anchorElem={floatingAnchorElem}
              cellMerge={true}
            />
            <FloatingTextFormatToolbarPlugin
              anchorElem={floatingAnchorElem}
            /> */}
            </>
          )}
        </div>
        <ActionsPlugin className='absolute bottom-4 right-4' />
      </LexicalComposer>
    </div>
  );
};

export default Editor;
