import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { $createCodeNode } from "@lexical/code";
import {
  INSERT_CHECK_LIST_COMMAND,
  INSERT_ORDERED_LIST_COMMAND,
  INSERT_UNORDERED_LIST_COMMAND,
  REMOVE_LIST_COMMAND,
} from "@lexical/list";
import {
  $createHeadingNode,
  $createQuoteNode,
  HeadingTagType,
} from "@lexical/rich-text";
import { $setBlocksType } from "@lexical/selection";
import {
  $createParagraphNode,
  $getSelection,
  $isRangeSelection,
  LexicalEditor,
} from "lexical";
import {
  Code,
  Heading1,
  Heading2,
  Heading3,
  Heading4,
  Heading5,
  Heading6,
  List,
  ListChecks,
  ListOrdered,
  Quote,
  Text,
} from "lucide-react";
import { blockTypeToBlockName, rootTypeToRootName } from "../utils/editing";

const BlockFormatDropDown = ({
  editor,
  blockType,
  rootType,
}: {
  blockType: keyof typeof blockTypeToBlockName;
  rootType: keyof typeof rootTypeToRootName;
  editor: LexicalEditor;
}) => {
  const formatParagraph = () => {
    if (blockType !== "paragraph") {
      editor.update(() => {
        const selection = $getSelection();
        $setBlocksType(selection, () => $createParagraphNode());
      });
    }
  };

  const formatHeading = (headingSize: HeadingTagType) => {
    if (blockType !== headingSize) {
      editor.update(() => {
        const selection = $getSelection();
        $setBlocksType(selection, () => $createHeadingNode(headingSize));
      });
    }
  };

  const formatBulletList = () => {
    if (blockType !== "bullet") {
      editor.dispatchCommand(INSERT_UNORDERED_LIST_COMMAND, undefined);
    } else {
      editor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
    }
  };

  const formatCheckList = () => {
    if (blockType !== "check") {
      editor.dispatchCommand(INSERT_CHECK_LIST_COMMAND, undefined);
    } else {
      editor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
    }
  };

  const formatNumberedList = () => {
    if (blockType !== "number") {
      editor.dispatchCommand(INSERT_ORDERED_LIST_COMMAND, undefined);
    } else {
      editor.dispatchCommand(REMOVE_LIST_COMMAND, undefined);
    }
  };

  const formatQuote = () => {
    if (blockType !== "quote") {
      editor.update(() => {
        const selection = $getSelection();
        $setBlocksType(selection, () => $createQuoteNode());
      });
    }
  };

  const formatCode = () => {
    if (blockType !== "code") {
      editor.update(() => {
        let selection = $getSelection();

        if (selection !== null) {
          if (selection.isCollapsed()) {
            $setBlocksType(selection, () => $createCodeNode());
          } else {
            const textContent = selection.getTextContent();
            const codeNode = $createCodeNode();
            selection.insertNodes([codeNode]);
            selection = $getSelection();
            if ($isRangeSelection(selection))
              selection.insertRawText(textContent);
          }
        }
      });
    }
  };

  return (
    <Select
      required
      value={blockType}
      onValueChange={(value) => {
        console.log(value);
        if (value === "paragraph") {
          return formatParagraph();
        }
        if (
          value === "h1" ||
          value === "h2" ||
          value === "h3" ||
          value === "h4" ||
          value === "h5" ||
          value === "h6"
        ) {
          return formatHeading(value);
        }
        if (value === "bullet") {
          return formatBulletList();
        }
        if (value === "number") {
          return formatNumberedList();
        }
        if (value === "check") {
          return formatCheckList();
        }
        if (value === "quote") {
          return formatQuote();
        }
        if (value === "code") {
          return formatCode();
        }
      }}
    >
      <SelectTrigger className="w-48">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="paragraph">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Text />
            <span className="text-nowrap">Normal</span>
          </div>
        </SelectItem>
        <SelectItem value="h1">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading1 className="icon h1" />
            <span className="text-nowrap">Heading 1</span>
          </div>
        </SelectItem>
        <SelectItem value="h2">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading2 />
            <span className="text-nowrap">Heading 2</span>
          </div>
        </SelectItem>
        <SelectItem value="h3">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading3 />
            <span className="text-nowrap">Heading 3</span>
          </div>
        </SelectItem>
        <SelectItem value="h4">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading4 />
            <span className="text-nowrap">Heading 4</span>
          </div>
        </SelectItem>
        <SelectItem value="h5">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading5 />
            <span className="text-nowrap">Heading 5</span>
          </div>
        </SelectItem>
        <SelectItem value="h6">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Heading6 />
            <span className="text-nowrap">Heading 6</span>
          </div>
        </SelectItem>
        <SelectItem value="bullet">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <List />
            <span className="text-nowrap">Bullet List</span>
          </div>
        </SelectItem>
        <SelectItem value="number">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <ListOrdered />
            <span className="text-nowrap">Numbered List</span>
          </div>
        </SelectItem>
        <SelectItem value="check">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <ListChecks />
            <span className="text-nowrap">Check List</span>
          </div>
        </SelectItem>
        <SelectItem value="quote">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Quote />
            <span className="text-nowrap">Quote</span>
          </div>
        </SelectItem>
        <SelectItem value="code">
          <div className="inline-flex flex-row items-center gap-2 px-2">
            <Code />
            <span className="text-nowrap">Code Block</span>
          </div>
        </SelectItem>
      </SelectContent>
    </Select>
  );
};

export default BlockFormatDropDown;

/* import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import {
  blockTypeToBlockName,
  rootTypeToRootName,
} from '@/shared';
import { $createCodeNode } from '@lexical/code';
import {
  INSERT_CHECK_LIST_COMMAND,
  INSERT_ORDERED_LIST_COMMAND,
  INSERT_UNORDERED_LIST_COMMAND,
  REMOVE_LIST_COMMAND,
} from '@lexical/list';
import {
  $createHeadingNode,
  $createQuoteNode,
  HeadingTagType,
} from '@lexical/rich-text';
import { $setBlocksType } from '@lexical/selection';
import {
  $createParagraphNode,
  $getSelection,
  $isRangeSelection,
  LexicalEditor,
} from 'lexical';
import {
  Code,
  Heading1,
  Heading2,
  Heading3,
  List,
  ListChecks,
  ListOrdered,
  Quote,
  Text,
} from 'lucide-react';

const BlockFormatDropDown = ({
  editor,
  blockType,
  rootType,
}: {
  blockType: keyof typeof blockTypeToBlockName;
  rootType: keyof typeof rootTypeToRootName;
  editor: LexicalEditor;
}) => {
  const formatParagraph = () => {
    editor.update(() => {
      const selection = $getSelection();
      $setBlocksType(selection, () =>
        $createParagraphNode()
      );
    });
  };

  const formatHeading = (headingSize: HeadingTagType) => {
    if (blockType !== headingSize) {
      editor.update(() => {
        const selection = $getSelection();
        $setBlocksType(selection, () =>
          $createHeadingNode(headingSize)
        );
      });
    }
  };

  const formatBulletList = () => {
    if (blockType !== 'bullet') {
      editor.dispatchCommand(
        INSERT_UNORDERED_LIST_COMMAND,
        undefined
      );
    } else {
      editor.dispatchCommand(
        REMOVE_LIST_COMMAND,
        undefined
      );
    }
  };

  const formatCheckList = () => {
    if (blockType !== 'check') {
      editor.dispatchCommand(
        INSERT_CHECK_LIST_COMMAND,
        undefined
      );
    } else {
      editor.dispatchCommand(
        REMOVE_LIST_COMMAND,
        undefined
      );
    }
  };

  const formatNumberedList = () => {
    if (blockType !== 'number') {
      editor.dispatchCommand(
        INSERT_ORDERED_LIST_COMMAND,
        undefined
      );
    } else {
      editor.dispatchCommand(
        REMOVE_LIST_COMMAND,
        undefined
      );
    }
  };

  const formatQuote = () => {
    if (blockType !== 'quote') {
      editor.update(() => {
        const selection = $getSelection();
        $setBlocksType(selection, () => $createQuoteNode());
      });
    }
  };

  const formatCode = () => {
    if (blockType !== 'code') {
      editor.update(() => {
        let selection = $getSelection();

        if (selection !== null) {
          if (selection.isCollapsed()) {
            $setBlocksType(selection, () =>
              $createCodeNode()
            );
          } else {
            const textContent = selection.getTextContent();
            const codeNode = $createCodeNode();
            selection.insertNodes([codeNode]);
            selection = $getSelection();
            if ($isRangeSelection(selection))
              selection.insertRawText(textContent);
          }
        }
      });
    }
  };

  const formatBlock = [
    {
      icon: Text,
      value: 'paragraph',
      function: formatParagraph,
      title: 'Normal',
    },
    {
      icon: Heading1,
      value: 'h1',
      function: formatHeading('h1'),
      title: 'Heading 1',
    },
    {
      icon: Heading2,
      value: 'h2',
      function: formatHeading('h2'),
      title: 'Heading 2',
    },
    {
      icon: Heading3,
      value: 'h3',
      function: formatHeading('h3'),
      title: 'Heading 3',
    },
    {
      icon: List,
      value: 'bullet',
      function: formatBulletList,
      title: 'Bullet List',
    },
    {
      icon: ListOrdered,
      value: 'number',
      function: formatNumberedList,
      title: 'Numbered List',
    },
    {
      icon: ListChecks,
      value: 'check',
      function: formatCheckList,
      title: 'Check List',
    },
    {
      icon: Quote,
      value: 'quote',
      function: formatQuote,
      title: 'Quote',
    },
    {
      icon: Code,
      value: 'code',
      function: formatCode,
      title: 'Code Block',
    },
  ];

  return (
    <Select
      required
      value={blockType}
      defaultValue={formatBlock.at(0)!.value}
      // onValueChange={(value) =>
      //   formatBlock.find((v) => v.value === value)!.function
      // }
    >
      <SelectTrigger className='w-48'>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {formatBlock.map((e) => (
          <SelectItem
            key={e.value}
            value={e.value}
          >
            <div className='inline-flex flex-row items-center gap-2 px-2'>
              <e.icon />
              <span className='text-nowrap'>{e.title}</span>
            </div>
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export default BlockFormatDropDown;
*/
