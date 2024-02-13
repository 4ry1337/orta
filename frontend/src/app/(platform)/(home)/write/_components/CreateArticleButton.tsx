'use client';
import { createarticle } from '@/app/actions/article/create';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { useAction } from '@/hooks/useAction';
import { useRouter } from 'next/navigation';
import { HTMLAttributes, useState } from 'react';

interface CreateArticleButtonProps
  extends HTMLAttributes<HTMLButtonElement> {
  user_id: string;
}

const CreateArticleButton = ({
  user_id,
  ...props
}: CreateArticleButtonProps) => {
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();
  const { toast } = useToast();
  const { execute } = useAction(createarticle, {
    onError: (error) => {
      toast({
        variant: 'destructive',
        title: error.status,
        description: error.message,
      });
    },
    onSuccess(data) {
      toast({
        title: data,
      });
    },
    onComplete() {
      setIsLoading(false);
    },
  });
  const CreateArticle = () => {
    setIsLoading(true);
    execute({ user_id }).catch((e) => console.log(e));
    router.push(`/${1}/${1}`);
  };
  return (
    <Button
      {...props}
      size={'lg'}
      onClick={() => CreateArticle()}
    >
      New article
    </Button>
  );
};

export default CreateArticleButton;
