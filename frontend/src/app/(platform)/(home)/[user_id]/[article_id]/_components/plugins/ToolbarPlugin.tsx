'use client';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
  ScrollArea,
  ScrollBar,
} from '@/components/ui/scroll-area';
import { Toggle } from '@/components/ui/toggle';
import { cn } from '@/lib/utils';
import {
  blockTypeToBlockName,
  rootTypeToRootName,
} from '@/shared';
import {
  $isCodeNode,
  CODE_LANGUAGE_FRIENDLY_NAME_MAP,
  CODE_LANGUAGE_MAP,
} from '@lexical/code';
import {
  $isLinkNode,
  TOGGLE_LINK_COMMAND,
} from '@lexical/link';
import { $isListNode, ListNode } from '@lexical/list';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $isDecoratorBlockNode } from '@lexical/react/LexicalDecoratorBlockNode';
import { INSERT_HORIZONTAL_RULE_COMMAND } from '@lexical/react/LexicalHorizontalRuleNode';
import {
  $isHeadingNode,
  $isQuoteNode,
} from '@lexical/rich-text';
import {
  $isParentElementRTL,
  $patchStyleText,
} from '@lexical/selection';
import { $isTableNode } from '@lexical/table';
import {
  $findMatchingParent,
  $getNearestBlockElementAncestorOrThrow,
  $getNearestNodeOfType,
  mergeRegister,
} from '@lexical/utils';
import {
  $createParagraphNode,
  $getNodeByKey,
  $getSelection,
  $isElementNode,
  $isRangeSelection,
  $isRootOrShadowRoot,
  $isTextNode,
  CAN_REDO_COMMAND,
  CAN_UNDO_COMMAND,
  COMMAND_PRIORITY_CRITICAL,
  COMMAND_PRIORITY_NORMAL,
  ElementFormatType,
  FORMAT_TEXT_COMMAND,
  KEY_MODIFIER_COMMAND,
  NodeKey,
  REDO_COMMAND,
  SELECTION_CHANGE_COMMAND,
  UNDO_COMMAND,
} from 'lexical';
import {
  Bold,
  CaseSensitive,
  Code,
  ImageIcon,
  Italic,
  Link,
  Plus,
  Redo,
  SeparatorHorizontal,
  Strikethrough,
  Subscript,
  Superscript,
  Trash2,
  Underline,
  Undo,
} from 'lucide-react';
import {
  Dispatch,
  HTMLAttributes,
  useCallback,
  useEffect,
  useState,
} from 'react';
import { getSelectedNode } from '../utils/getSelectedNode';
import { sanitizeUrl } from '../utils/url';
import BlockFormatSelect from './BlockFormatSelect';
import ElementFormatSelect from './ElementFormatDropdown';

function getCodeLanguageOptions(): [string, string][] {
  const options: [string, string][] = [];

  for (const [lang, friendlyName] of Object.entries(
    CODE_LANGUAGE_FRIENDLY_NAME_MAP
  )) {
    options.push([lang, friendlyName]);
  }

  return options;
}

const CODE_LANGUAGE_OPTIONS = getCodeLanguageOptions();

interface ToolbarPluginProps
  extends HTMLAttributes<HTMLDivElement> {
  setIsLinkEditMode: Dispatch<boolean>;
}

const ToolbarPlugin = ({
  setIsLinkEditMode,
  className,
  ...props
}: ToolbarPluginProps) => {
  const [editor] = useLexicalComposerContext();
  const [activeEditor, setActiveEditor] = useState(editor);
  const [blockType, setBlockType] =
    useState<keyof typeof blockTypeToBlockName>(
      'paragraph'
    );
  const [rootType, setRootType] =
    useState<keyof typeof rootTypeToRootName>('root');
  const [selectedElementKey, setSelectedElementKey] =
    useState<NodeKey | null>(null);
  const [fontSize, setFontSize] = useState<string>('15px');
  const [fontFamily, setFontFamily] =
    useState<string>('Arial');
  const [elementFormat, setElementFormat] =
    useState<ElementFormatType>('left');
  const [isLink, setIsLink] = useState(false);
  const [isBold, setIsBold] = useState(false);
  const [isItalic, setIsItalic] = useState(false);
  const [isUnderline, setIsUnderline] = useState(false);
  const [isStrikethrough, setIsStrikethrough] =
    useState(false);
  const [isSubscript, setIsSubscript] = useState(false);
  const [isSuperscript, setIsSuperscript] = useState(false);
  const [isCode, setIsCode] = useState(false);
  const [canUndo, setCanUndo] = useState(false);
  const [canRedo, setCanRedo] = useState(false);
  const [isRTL, setIsRTL] = useState(false);
  const [codeLanguage, setCodeLanguage] =
    useState<string>('');
  const [isEditable, setIsEditable] = useState(() =>
    editor.isEditable()
  );

  const $updateToolbar = useCallback(() => {
    const selection = $getSelection();
    if ($isRangeSelection(selection)) {
      const anchorNode = selection.anchor.getNode();
      let element =
        anchorNode.getKey() === 'root'
          ? anchorNode
          : $findMatchingParent(anchorNode, (e) => {
              const parent = e.getParent();
              return (
                parent !== null &&
                $isRootOrShadowRoot(parent)
              );
            });

      if (element === null) {
        element = anchorNode.getTopLevelElementOrThrow();
      }

      const elementKey = element.getKey();
      const elementDOM =
        activeEditor.getElementByKey(elementKey);

      // Update text format
      setIsBold(selection.hasFormat('bold'));
      setIsItalic(selection.hasFormat('italic'));
      setIsUnderline(selection.hasFormat('underline'));
      setIsStrikethrough(
        selection.hasFormat('strikethrough')
      );
      setIsSubscript(selection.hasFormat('subscript'));
      setIsSuperscript(selection.hasFormat('superscript'));
      setIsCode(selection.hasFormat('code'));
      setIsRTL($isParentElementRTL(selection));

      // Update links
      const node = getSelectedNode(selection);
      const parent = node.getParent();
      if ($isLinkNode(parent) || $isLinkNode(node)) {
        setIsLink(true);
      } else {
        setIsLink(false);
      }

      const tableNode = $findMatchingParent(
        node,
        $isTableNode
      );
      if ($isTableNode(tableNode)) {
        setRootType('table');
      } else {
        setRootType('root');
      }

      if (elementDOM !== null) {
        setSelectedElementKey(elementKey);
        if ($isListNode(element)) {
          const parentList =
            $getNearestNodeOfType<ListNode>(
              anchorNode,
              ListNode
            );
          const type = parentList
            ? parentList.getListType()
            : element.getListType();
          setBlockType(type);
        } else {
          const type = $isHeadingNode(element)
            ? element.getTag()
            : element.getType();
          if (type in blockTypeToBlockName) {
            setBlockType(
              type as keyof typeof blockTypeToBlockName
            );
          }
          if ($isCodeNode(element)) {
            const language =
              element.getLanguage() as keyof typeof CODE_LANGUAGE_MAP;
            setCodeLanguage(
              language
                ? CODE_LANGUAGE_MAP[language] || language
                : ''
            );
            return;
          }
        }
      }
      // Handle buttons
      let matchingParent;
      if ($isLinkNode(parent)) {
        // If node is a link, we need to fetch the parent paragraph node to set format
        matchingParent = $findMatchingParent(
          node,
          (parentNode) =>
            $isElementNode(parentNode) &&
            !parentNode.isInline()
        );
      }

      // If matchingParent is a valid node, pass it's format type
      setElementFormat(
        $isElementNode(matchingParent)
          ? matchingParent.getFormatType()
          : $isElementNode(node)
            ? node.getFormatType()
            : parent?.getFormatType() || 'left'
      );
    }
  }, [activeEditor]);

  useEffect(() => {
    return editor.registerCommand(
      SELECTION_CHANGE_COMMAND,
      (_payload, newEditor) => {
        $updateToolbar();
        setActiveEditor(newEditor);
        return false;
      },
      COMMAND_PRIORITY_CRITICAL
    );
  }, [editor, $updateToolbar]);

  useEffect(() => {
    return mergeRegister(
      editor.registerEditableListener((editable) => {
        setIsEditable(editable);
      }),
      activeEditor.registerUpdateListener(
        ({ editorState }) => {
          editorState.read(() => {
            $updateToolbar();
          });
        }
      ),
      activeEditor.registerCommand<boolean>(
        CAN_UNDO_COMMAND,
        (payload) => {
          setCanUndo(payload);
          return false;
        },
        COMMAND_PRIORITY_CRITICAL
      ),
      activeEditor.registerCommand<boolean>(
        CAN_REDO_COMMAND,
        (payload) => {
          setCanRedo(payload);
          return false;
        },
        COMMAND_PRIORITY_CRITICAL
      )
    );
  }, [$updateToolbar, activeEditor, editor]);

  useEffect(() => {
    return activeEditor.registerCommand(
      KEY_MODIFIER_COMMAND,
      (payload) => {
        const event: KeyboardEvent = payload;
        const { code, ctrlKey, metaKey } = event;

        if (code === 'KeyK' && (ctrlKey || metaKey)) {
          event.preventDefault();
          let url: string | null;
          if (!isLink) {
            setIsLinkEditMode(true);
            url = sanitizeUrl('https://');
          } else {
            setIsLinkEditMode(false);
            url = null;
          }
          return activeEditor.dispatchCommand(
            TOGGLE_LINK_COMMAND,
            url
          );
        }
        return false;
      },
      COMMAND_PRIORITY_NORMAL
    );
  }, [activeEditor, isLink, setIsLinkEditMode]);

  const applyStyleText = useCallback(
    (
      styles: Record<string, string>,
      skipHistoryStack?: boolean
    ) => {
      activeEditor.update(
        () => {
          const selection = $getSelection();
          if (selection !== null) {
            $patchStyleText(selection, styles);
          }
        },
        skipHistoryStack ? { tag: 'historic' } : {}
      );
    },
    [activeEditor]
  );

  const clearFormatting = useCallback(() => {
    activeEditor.update(() => {
      const selection = $getSelection();
      if ($isRangeSelection(selection)) {
        const anchor = selection.anchor;
        const focus = selection.focus;
        const nodes = selection.getNodes();

        if (
          anchor.key === focus.key &&
          anchor.offset === focus.offset
        ) {
          return;
        }

        nodes.forEach((node, idx) => {
          // We split the first and last node by the selection
          // So that we don't format unselected text inside those nodes
          if ($isTextNode(node)) {
            // Use a separate variable to ensure TS does not lose the refinement
            let textNode = node;
            if (idx === 0 && anchor.offset !== 0) {
              textNode =
                textNode.splitText(anchor.offset)[1] ||
                textNode;
            }
            if (idx === nodes.length - 1) {
              textNode =
                textNode.splitText(focus.offset)[0] ||
                textNode;
            }

            if (textNode.__style !== '') {
              textNode.setStyle('');
            }
            if (textNode.__format !== 0) {
              textNode.setFormat(0);
              $getNearestBlockElementAncestorOrThrow(
                textNode
              ).setFormat('');
            }
            node = textNode;
          } else if (
            $isHeadingNode(node) ||
            $isQuoteNode(node)
          ) {
            node.replace($createParagraphNode(), true);
          } else if ($isDecoratorBlockNode(node)) {
            node.setFormat('');
          }
        });
      }
    });
  }, [activeEditor]);

  const insertLink = useCallback(() => {
    if (!isLink) {
      setIsLinkEditMode(true);
      editor.dispatchCommand(
        TOGGLE_LINK_COMMAND,
        sanitizeUrl('https://')
      );
    } else {
      setIsLinkEditMode(false);
      editor.dispatchCommand(TOGGLE_LINK_COMMAND, null);
    }
  }, [editor, isLink, setIsLinkEditMode]);

  const onCodeLanguageSelect = useCallback(
    (value: string) => {
      activeEditor.update(() => {
        if (selectedElementKey !== null) {
          const node = $getNodeByKey(selectedElementKey);
          if ($isCodeNode(node)) {
            node.setLanguage(value);
          }
        }
      });
    },
    [activeEditor, selectedElementKey]
  );
  // const insertGifOnClick = (
  //   payload: InsertImagePayload
  // ) => {
  //   activeEditor.dispatchCommand(
  //     INSERT_EMBED_COMMAND,
  //     payload
  //   );
  // };

  return (
    <div
      className={cn(className, 'rounded-md border')}
      {...props}
    >
      <ScrollArea>
        <div className='inline-flex flex-row items-center justify-start gap-4 p-3'>
          <div className='inline-flex flex-row items-center justify-start gap-1'>
            <Button
              size={'icon'}
              variant={'ghost'}
              disabled={!canUndo}
              onClick={() => {
                activeEditor.dispatchCommand(
                  UNDO_COMMAND,
                  undefined
                );
              }}
              //title={IS_APPLE ? 'Undo (⌘Z)' : 'Undo (Ctrl+Z)'}
              aria-label='Undo'
            >
              <Undo />
            </Button>
            <Button
              size={'icon'}
              variant={'ghost'}
              disabled={!canRedo}
              onClick={() => {
                activeEditor.dispatchCommand(
                  REDO_COMMAND,
                  undefined
                );
              }}
              //title={IS_APPLE ? 'Redo (⌘Y)' : 'Redo (Ctrl+Y)'}
              aria-label='Redo'
            >
              <Redo />
            </Button>
          </div>
          <div className='inline-flex flex-row items-center justify-start gap-1'>
            <BlockFormatSelect
              blockType={blockType}
              rootType={rootType}
              editor={editor}
            />
          </div>
          <div className='inline-flex flex-row items-center justify-start gap-1'>
            <Toggle
              pressed={isBold}
              onClick={() => {
                activeEditor.dispatchCommand(
                  FORMAT_TEXT_COMMAND,
                  'bold'
                );
              }}
              asChild
              // title={IS_APPLE ? 'Bold (⌘B)' : 'Bold (Ctrl+B)'}
              // aria-label={`Format text as bold. Shortcut: ${
              //   IS_APPLE ? '⌘B' : 'Ctrl+B'
              // }`}
            >
              <Button
                variant={'secondary'}
                size={'icon'}
              >
                <Bold />
              </Button>
            </Toggle>
            <Toggle
              pressed={isItalic}
              onClick={() => {
                activeEditor.dispatchCommand(
                  FORMAT_TEXT_COMMAND,
                  'italic'
                );
              }}
              asChild
              // title={
              //   IS_APPLE ? 'Italic (⌘I)' : 'Italic (Ctrl+I)'
              // }
              // aria-label={`Format text as italics. Shortcut: ${
              //   IS_APPLE ? '⌘I' : 'Ctrl+I'
              // }`}
            >
              <Button
                variant={'secondary'}
                size={'icon'}
              >
                <Italic />
              </Button>
            </Toggle>
            <Toggle
              pressed={isUnderline}
              onClick={() => {
                activeEditor.dispatchCommand(
                  FORMAT_TEXT_COMMAND,
                  'underline'
                );
              }}
              asChild
              // title={
              //   IS_APPLE
              //     ? 'Underline (⌘U)'
              //     : 'Underline (Ctrl+U)'
              // }
              // aria-label={`Format text to underlined. Shortcut: ${
              //   IS_APPLE ? '⌘U' : 'Ctrl+U'
              // }`}
            >
              <Button
                variant={'secondary'}
                size={'icon'}
              >
                <Underline />
              </Button>
            </Toggle>
            <Toggle
              pressed={isCode}
              onClick={() => {
                activeEditor.dispatchCommand(
                  FORMAT_TEXT_COMMAND,
                  'code'
                );
              }}
              asChild
              title='Insert code block'
              aria-label='Insert code block'
            >
              <Button
                variant={'secondary'}
                size={'icon'}
              >
                <Code />
              </Button>
            </Toggle>
            <Toggle
              pressed={isLink}
              onClick={insertLink}
              aria-label='Insert link'
              title='Insert link'
              asChild
            >
              <Button
                variant={'secondary'}
                size={'icon'}
              >
                <Link />
              </Button>
            </Toggle>
            <DropdownMenu>
              <DropdownMenuTrigger
                asChild
                aria-label='Formatting options for additional text styles'
              >
                <Button
                  variant={'ghost'}
                  size={'icon'}
                >
                  <CaseSensitive />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent>
                <DropdownMenuItem
                  onClick={() => {
                    activeEditor.dispatchCommand(
                      FORMAT_TEXT_COMMAND,
                      'strikethrough'
                    );
                  }}
                  title='Strikethrough'
                  className='flex flex-row items-center gap-2 px-2'
                  aria-label='Format text with a strikethrough'
                >
                  <Strikethrough />
                  <span>Strikethrough</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  onClick={() => {
                    activeEditor.dispatchCommand(
                      FORMAT_TEXT_COMMAND,
                      'subscript'
                    );
                  }}
                  title='Subscript'
                  className='flex flex-row items-center gap-2 px-2'
                  aria-label='Format text with a subscript'
                >
                  <Subscript />
                  <span>Subscript</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  onClick={() => {
                    activeEditor.dispatchCommand(
                      FORMAT_TEXT_COMMAND,
                      'superscript'
                    );
                  }}
                  title='Superscript'
                  className='flex flex-row items-center gap-2 px-2'
                  aria-label='Format text with a superscript'
                >
                  <Superscript />
                  <span>Superscript</span>
                </DropdownMenuItem>
                <DropdownMenuItem
                  onClick={clearFormatting}
                  className='flex flex-row items-center gap-2 px-2'
                  title='Clear text formatting'
                  aria-label='Clear all text formatting'
                >
                  <Trash2 />
                  <span>Clear Formatting</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
          <div className='inline-flex flex-row items-center justify-start gap-1'>
            <DropdownMenu>
              <DropdownMenuTrigger
                aria-label='Insert specialized editor node'
                asChild
              >
                <Button
                  variant={'ghost'}
                  className='gap-2'
                >
                  <Plus />
                  <span>Insert</span>
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent>
                <DropdownMenuItem
                  onClick={() => {
                    activeEditor.dispatchCommand(
                      INSERT_HORIZONTAL_RULE_COMMAND,
                      undefined
                    );
                  }}
                >
                  <div className='inline-flex flex-row items-center gap-2 px-2'>
                    <SeparatorHorizontal />
                    <span>Horizontal Rule</span>
                  </div>
                </DropdownMenuItem>
                <DropdownMenuItem
                // onClick={() => {
                //   showModal('Insert Image', (onClose) => (
                //     <InsertImageDialog
                //       activeEditor={activeEditor}
                //       onClose={onClose}
                //     />
                //   ));
                // }}
                >
                  <div className='inline-flex flex-row items-center gap-2 px-2'>
                    <ImageIcon />
                    <span>Image</span>
                  </div>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
          <div className='inline-flex flex-row items-center justify-start gap-1'>
            <ElementFormatSelect
              value={elementFormat}
              editor={editor}
              isRTL={isRTL}
            />
          </div>
        </div>
        <ScrollBar orientation='horizontal' />
      </ScrollArea>
    </div>
  );
};

export default ToolbarPlugin;
