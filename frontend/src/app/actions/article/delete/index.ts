'use server';

import { createSafeAction } from '@/lib/create-safe-action';
import { deleteArticleSchema } from './schema';
import { InputType, ReturnType } from './type';
import { revalidatePath } from 'next/cache';

const handler = async (
  input: InputType
): Promise<ReturnType> => {
  const res = await fetch(
    `http://127.0.0.1:5000/api/article/${input.article_id}`,
    {
      method: 'DELETE',
    }
  );
  let data: string = await res.json();
  if (res.ok) {
    revalidatePath(`/write`);
    return {
      data,
    };
  }
  return {
    error: {
      message: data,
      status: String(res.status),
    },
  };
};

export const deletearticle = createSafeAction(
  deleteArticleSchema,
  handler
);
