import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Separator } from '@/components/ui/separator';
import { ELEMENT_FORMAT_OPTIONS } from '@/shared';
import {
  ElementFormatType,
  FORMAT_ELEMENT_COMMAND,
  INDENT_CONTENT_COMMAND,
  LexicalEditor,
  OUTDENT_CONTENT_COMMAND,
} from 'lexical';
import {
  AlignCenter,
  AlignJustify,
  AlignLeft,
  AlignRight,
  Indent,
  Outdent,
} from 'lucide-react';

// const ElementFormatOptions = [
//   {
//     value = 'left',
//     onClick =
//   },
// ];

const ElementFormatSelect = ({
  editor,
  value,
  isRTL,
}: {
  editor: LexicalEditor;
  value: ElementFormatType;
  isRTL: boolean;
}) => {
  const formatOption =
    ELEMENT_FORMAT_OPTIONS[value || 'left'];

  return (
    <Select
      required={true}
      name='Formatting Options'
      value={value}
      onValueChange={(
        value: ElementFormatType | 'indent' | 'outdent'
      ) => {
        if (value === 'outdent') {
          return editor.dispatchCommand(
            OUTDENT_CONTENT_COMMAND,
            undefined
          );
        } else if (value === 'indent') {
          editor.dispatchCommand(
            INDENT_CONTENT_COMMAND,
            undefined
          );
        } else {
          return editor.dispatchCommand(
            FORMAT_ELEMENT_COMMAND,
            value
          );
        }
      }}
    >
      <SelectTrigger
        className='w-48'
        aria-label='Formatting options for text alignment'
      >
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value='left'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            <AlignLeft className='' />
            <span className='text-nowrap'>Left Align</span>
          </div>
        </SelectItem>
        <SelectItem value='center'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            <AlignCenter />
            <span className='text-nowrap'>
              Center Align
            </span>
          </div>
        </SelectItem>
        <SelectItem value='right'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            <AlignRight />
            <span className='text-nowrap'>Right Align</span>
          </div>
        </SelectItem>
        <SelectItem value='justify'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            <AlignJustify />
            <span className='text-nowrap'>
              Justify Align
            </span>
          </div>
        </SelectItem>
        <SelectItem value='start'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            {isRTL ? (
              <ELEMENT_FORMAT_OPTIONS.start.iconRTL />
            ) : (
              <ELEMENT_FORMAT_OPTIONS.start.icon />
            )}
            <span className='text-nowrap'>Start Align</span>
          </div>
        </SelectItem>
        <SelectItem value='end'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            {isRTL ? (
              <ELEMENT_FORMAT_OPTIONS.end.iconRTL />
            ) : (
              <ELEMENT_FORMAT_OPTIONS.end.icon />
            )}
            <span className='text-nowrap'>End Align</span>
          </div>
        </SelectItem>
        <Separator />
        <SelectItem value='outdent'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            {isRTL ? <Indent /> : <Outdent />}
            <span className='text-nowrap'>Outdent</span>
          </div>
        </SelectItem>
        <SelectItem value='indent'>
          <div className='inline-flex flex-row items-center gap-2 px-2'>
            {isRTL ? <Outdent /> : <Indent />}
            <span className='text-nowrap'>Indent</span>
          </div>
        </SelectItem>
      </SelectContent>
    </Select>
  );
};

export default ElementFormatSelect;
