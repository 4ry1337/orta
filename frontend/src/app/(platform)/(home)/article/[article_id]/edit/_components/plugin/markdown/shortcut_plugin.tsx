import { MarkdownShortcutPlugin } from "@lexical/react/LexicalMarkdownShortcutPlugin";
import * as React from "react";

import { EDITOR_TRANSFORMERS } from "./transformers";

const MarkdownPlugin = () => {
  return <MarkdownShortcutPlugin transformers={EDITOR_TRANSFORMERS} />;
};

export default MarkdownPlugin;
