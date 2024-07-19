import { Separator } from "@/components/ui/separator";
import { HTMLAttributes } from "react";
import ArticleForm from "./forms/article_form";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import CollaboratorForm from "./forms/collaborator_form";
import TagsForm from "./forms/tags_form";
interface ArticleSettingsTabProps extends HTMLAttributes<HTMLDivElement> { }

const ArticleSettingsTab = ({ }: ArticleSettingsTabProps) => {
  return (
    <div className="mx-auto space-y-4 max-w-lg">
      <div className="space-y-2">
        <h3 className="text-lg font-medium">Article</h3>
        <p className="text-sm text-muted-foreground">
          Update your article settings. Set your preferred language and tags.
        </p>
      </div>
      <Separator />
      <Tabs defaultValue="article">
        <TabsList>
          <TabsTrigger value="article">Article</TabsTrigger>
          <TabsTrigger value="collaboration">Collaborator</TabsTrigger>
        </TabsList>
        <TabsContent value="article">
          <ArticleForm />
        </TabsContent>
        <TabsContent value="collaboration">
          <CollaboratorForm />
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default ArticleSettingsTab;
