import { LinkPlugin as LexicalLinkPlugin } from "@lexical/react/LexicalLinkPlugin";
import { validateUrl } from "../utils/url";

const LinkPlugin = () => {
  return <LexicalLinkPlugin validateUrl={validateUrl} />;
};

export default LinkPlugin;
