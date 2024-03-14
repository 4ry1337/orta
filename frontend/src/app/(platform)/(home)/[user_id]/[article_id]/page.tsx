import db from '@/lib/prismadb';
import Editor from './_components/Editor';
import { Article, ArticleVersion } from '@prisma/client';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { z } from 'zod';
import { Separator } from '@/components/ui/separator';
import { useForm } from 'react-hook-form';
import ArticleSettings from './_components/ArticleSettings';
import { toast } from '@/components/ui/use-toast';
import { redirect } from 'next/navigation';
interface IParams {
  user_id: string;
  article_id: string;
}

const getArticle = async (user_id: string, article_id: number): Promise<Article> => {
  const article = await db.article.findUnique({
    where: {
      id: article_id,
      userIds: {
        has: user_id,
      }
    }
  })
  if (!article) {
    return redirect(`/${user_id}`)
  }
  return article;
}


const getArticleVersion = async (article_id: number): Promise<ArticleVersion> => {
  const data = await db.articleVersion.findFirst({
    where: {
      article_id: article_id,
    },
    orderBy: {
      version_number: 'desc'
    }
  })
  if (!data) {
    return await db.articleVersion.create({
      data: {
        article_id: article_id,
        content: null
      }
    })
  }
  console.log(data)
  return data;
}

const ArticlePage = async ({ params }: { params: IParams }) => {
  let articleVersion = await getArticleVersion(Number(params.article_id));
  let article: Article = await getArticle(params.user_id, Number(params.article_id));
  return (
    <Tabs defaultValue='article' className='flex flex-col h-full overflow-hidden p-4'>
      <TabsList className='grid grid-cols-3'>
        <TabsTrigger value='article'>
          Article
        </TabsTrigger>
        <TabsTrigger value='editor'>
          Editor
        </TabsTrigger>
        <TabsTrigger value='preview'>
          Preview
        </TabsTrigger>
      </TabsList>
      <TabsContent className='grow min-h-0' value='article'>
        <ArticleSettings article={article} />
      </TabsContent>
      <TabsContent className='grow min-h-0' value='editor'>
        <Editor className='h-full' article_version={articleVersion} />
      </TabsContent>
      <TabsContent className='grow min-h-0' value='preview'>
        <div>{article?.title}</div>
      </TabsContent>
    </Tabs>
  );
};

export default ArticlePage;
