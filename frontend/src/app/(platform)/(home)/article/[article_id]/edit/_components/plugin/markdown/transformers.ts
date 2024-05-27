import {
  CHECK_LIST,
  ELEMENT_TRANSFORMERS,
  ElementTransformer,
  TEXT_FORMAT_TRANSFORMERS,
  TEXT_MATCH_TRANSFORMERS,
  TextMatchTransformer,
  Transformer,
} from "@lexical/markdown";
import {
  $createHorizontalRuleNode,
  $isHorizontalRuleNode,
  HorizontalRuleNode,
} from "@lexical/react/LexicalHorizontalRuleNode";
import { LexicalNode } from "lexical";

export const HR: ElementTransformer = {
  dependencies: [HorizontalRuleNode],
  export: (node: LexicalNode) => {
    return $isHorizontalRuleNode(node) ? "***" : null;
  },
  regExp: /^(---|\*\*\*|___)\s?$/,
  replace: (parentNode, _1, _2, isImport) => {
    const line = $createHorizontalRuleNode();

    // TODO: Get rid of isImport flag
    if (isImport || parentNode.getNextSibling() != null) {
      parentNode.replace(line);
    } else {
      parentNode.insertBefore(line);
    }

    line.selectNext();
  },
  type: "element",
};

// export const IMAGE: TextMatchTransformer = {
//   dependencies: [ImageNode],
//   export: (node) => {
//     if (!$isImageNode(node)) {
//       return null;
//     }
//
//     return `![${node.getAltText()}](${node.getSrc()})`;
//   },
//   importRegExp: /!(?:\[([^[]*)\])(?:\(([^(]+)\))/,
//   regExp: /!(?:\[([^[]*)\])(?:\(([^(]+)\))$/,
//   replace: (textNode, match) => {
//     const [, altText, src] = match;
//     const imageNode = $createImageNode({
//       altText,
//       maxWidth: 800,
//       src,
//     });
//     textNode.replace(imageNode);
//   },
//   trigger: ")",
//   type: "text-match",
// };

export const EDITOR_TRANSFORMERS: Array<Transformer> = [
  HR,
  CHECK_LIST,
  ...ELEMENT_TRANSFORMERS,
  ...TEXT_FORMAT_TRANSFORMERS,
  ...TEXT_MATCH_TRANSFORMERS,
];
