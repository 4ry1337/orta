import Heading from "@tiptap/extension-heading";
import Blockquote from "@tiptap/extension-blockquote";
import OrderedList from "@tiptap/extension-ordered-list";
import ListItem from "@tiptap/extension-list-item";
import BulletList from "@tiptap/extension-bullet-list";
import Highlight from "@tiptap/extension-highlight";
import CodeBlock from "@tiptap/extension-code-block";
import Youtube from "@tiptap/extension-youtube";
import Code from "@tiptap/extension-code";
import Bold from "@tiptap/extension-bold";
import Italic from "@tiptap/extension-italic";
import Strike from "@tiptap/extension-strike";
import Subscript from "@tiptap/extension-subscript";
import Link from "@tiptap/extension-link";
import Superscript from "@tiptap/extension-superscript";
import Underline from "@tiptap/extension-underline";
import Document from "@tiptap/extension-document";
import Paragraph from "@tiptap/extension-paragraph";
import Text from "@tiptap/extension-text";
import Dropcursor from "@tiptap/extension-dropcursor";
import Image from "@tiptap/extension-image";

export const default_extensions = [
  Document,
  Paragraph,
  Text,
  CodeBlock,
  Blockquote,
  BulletList,
  OrderedList,
  ListItem,
  Heading,
  Bold,
  Italic,
  Code,
  Strike,
  Subscript,
  Superscript,
  Link,
  Underline,
  Highlight,
  Dropcursor,
  Image.configure({
    inline: true,
  }),
  Youtube.configure({
    allowFullscreen: true,
    nocookie: true,
    height: undefined,
    width: undefined,
    HTMLAttributes: {
      class: "w-full aspect-video",
    },
  }),
];
