'use client';
import { createarticle } from '@/app/actions/article/create';
import { Button } from '@/components/ui/button';
import { toast, useToast } from '@/components/ui/use-toast';
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
  const router = useRouter()
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
        title: 'Created'
      });
      router.push(`/${user_id}/${data.id}`);
    },
  });
  const CreateArticle = () => {
    execute({ user_id })
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
