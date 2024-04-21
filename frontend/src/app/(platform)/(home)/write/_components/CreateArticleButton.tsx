'use client';

import { Button } from '@/components/ui/button';
import { HTMLAttributes } from 'react';

interface CreateArticleButtonProps
  extends HTMLAttributes<HTMLButtonElement> {
  user_id: string;
}

const CreateArticleButton = ({
  user_id,
  ...props
}: CreateArticleButtonProps) => {
  return (
    <Button
      {...props}
      size={'lg'}
    >
      New article
    </Button>
  );
};

export default CreateArticleButton;
