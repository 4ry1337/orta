"use client";

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import ArticleSettingsTab from "./settings_tab";
import EditorTab from "./editor_tab";
import PreviewTab from "./preview_tab";
import { get_id } from "@/lib/utils";
import Link from "next/link";
import { ArticleProvider } from "@/context/article_context";

interface IParams {
  article_id: string;
}

const ArticleEditPage = ({ params }: { params: IParams }) => {
  return (
    <ArticleProvider article_id={get_id(params.article_id)}>
      <Tabs defaultValue={"editor"}>
        <div className="pt-6 px-3">
          <TabsList>
            <TabsTrigger value="editor">
              <Link href={"#editor"}>Editor</Link>
            </TabsTrigger>
            <TabsTrigger value="preview">
              <Link href={"#preview"}>Preview</Link>
            </TabsTrigger>
            <TabsTrigger value="article">
              <Link href={"#article"}>Article</Link>
            </TabsTrigger>
          </TabsList>
        </div>
        <TabsContent value="editor">
          <EditorTab />
        </TabsContent>
        <TabsContent value="preview">
          <PreviewTab />
        </TabsContent>
        <TabsContent value="article">
          <ArticleSettingsTab />
        </TabsContent>
      </Tabs>
    </ArticleProvider>
  );
};

export default ArticleEditPage;
