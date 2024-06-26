import { Hocuspocus } from "@hocuspocus/server";
import { TiptapTransformer } from "@hocuspocus/transformer";
import axios from "axios";
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
import { generateHTML, generateJSON } from "@tiptap/html";
import { Doc } from "yjs";
const server = new Hocuspocus({
    name: "orta-colab",
    port: 6565,
    onAuthenticate: async (data) => {
        return {
            token: data.token,
        };
    },
    onStoreDocument: async (data) => {
        await axios
            .patch(`http://localhost:5000/api/articles/${data.documentName}/edit`, {
            content: generateHTML(TiptapTransformer.fromYdoc(data.document, "default"), [
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
                Image,
                Youtube,
            ]),
        }, {
            headers: {
                Authorization: `Bearer ${data.context.token}`,
            },
        })
            .catch((err) => {
            console.log(err.message);
            return null;
        });
    },
    onLoadDocument: async (data) => {
        const article = await axios
            .get(`http://localhost:5000/api/articles/${data.documentName}`)
            .then((res) => res.data)
            .catch((err) => {
            console.error(err);
            return null;
        });
        const ydoc = article?.content
            ? TiptapTransformer.toYdoc(generateJSON(article.content, [
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
                Image,
                Youtube,
            ]), "default")
            : new Doc();
        return ydoc;
    },
});
server.listen(async (payload) => {
    console.log("Server is listening on: ", payload.port);
});
