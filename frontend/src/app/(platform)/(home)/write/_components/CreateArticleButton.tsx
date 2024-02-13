'use client';
import { Button } from '@/components/ui/button';
import { useRouter } from 'next/navigation';

const CreateArticleButton = () => {
  const router = useRouter();

  const CreateArticle = () => {
    router.push(`/${1}/${1}`);
  };
  return (
    <Button
      size={'lg'}
      onClick={() => CreateArticle()}
    >
      New article
    </Button>
  );
};

export default CreateArticleButton;
