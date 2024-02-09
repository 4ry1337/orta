'use client';

import { LexicalComposer } from '@lexical/react/LexicalComposer';
import Editor from './_components/Editor';

const ArticlePage = () => {
  const initialConfig = {
    // NOTE: This is critical for collaboration plugin to set editor state to null. It
    // would indicate that the editor should not try to set any default state
    // (not even empty one), and let collaboration plugin do it instead
    editorState: null,
    namespace: 'Article',
    nodes: [],
    onError: (error: Error) => {
      throw error;
    },
    theme: {},
  };
  return (
    <>
      <LexicalComposer initialConfig={initialConfig}>
        <Editor />
      </LexicalComposer>
      {/* <div className='flex h-full w-full flex-col p-4'>
        <Tabs defaultValue='write'>
          <TabsList className='grid w-full grid-cols-2'>
            <TabsTrigger value='metadata'>
              Article
            </TabsTrigger>
            <TabsTrigger value='write'>Content</TabsTrigger>
          </TabsList>
          <TabsContent value='metadata'>
            <div className='flex items-center justify-center'>
              NOT IMPLEMENTED
            </div>
          </TabsContent>
          <TabsContent value='write'>
            <Editor />
          </TabsContent>
        </Tabs>
      </div> */}
    </>
  );
};

export default ArticlePage;
