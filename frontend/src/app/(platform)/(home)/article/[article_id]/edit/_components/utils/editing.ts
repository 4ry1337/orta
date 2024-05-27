import { ElementFormatType } from "lexical";
import { AlignCenter, AlignJustify, AlignLeft, AlignRight } from "lucide-react";

export const rootTypeToRootName = {
  root: "Root",
  table: "Table",
};

export const blockTypeToBlockName = {
  bullet: "Bulleted List",
  check: "Check List",
  code: "Code Block",
  h1: "Heading 1",
  h2: "Heading 2",
  h3: "Heading 3",
  h4: "Heading 4",
  h5: "Heading 5",
  h6: "Heading 6",
  number: "Numbered List",
  paragraph: "Normal",
  quote: "Quote",
};

export const FONT_FAMILY_OPTIONS: [string, string][] = [
  ["Arial", "Arial"],
  ["Courier New", "Courier New"],
  ["Georgia", "Georgia"],
  ["Times New Roman", "Times New Roman"],
  ["Trebuchet MS", "Trebuchet MS"],
  ["Verdana", "Verdana"],
];

export const FONT_SIZE_OPTIONS: [string, string][] = [
  ["10px", "10px"],
  ["11px", "11px"],
  ["12px", "12px"],
  ["13px", "13px"],
  ["14px", "14px"],
  ["15px", "15px"],
  ["16px", "16px"],
  ["17px", "17px"],
  ["18px", "18px"],
  ["19px", "19px"],
  ["20px", "20px"],
];

export const ELEMENT_FORMAT_OPTIONS: {
  [key in Exclude<ElementFormatType, "">]: {
    icon: any;
    iconRTL: any;
    name: string;
  };
} = {
  center: {
    icon: AlignCenter,
    iconRTL: AlignCenter,
    name: "Center Align",
  },
  end: {
    icon: AlignRight,
    iconRTL: AlignLeft,
    name: "End Align",
  },
  justify: {
    icon: AlignJustify,
    iconRTL: AlignJustify,
    name: "Justify Align",
  },
  left: {
    icon: AlignLeft,
    iconRTL: AlignLeft,
    name: "Left Align",
  },
  right: {
    icon: AlignLeft,
    iconRTL: AlignRight,
    name: "Right Align",
  },
  start: {
    icon: AlignLeft,
    iconRTL: AlignRight,
    name: "Start Align",
  },
};
