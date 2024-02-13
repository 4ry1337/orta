'use server';

import { createSafeAction } from '@/lib/create-safe-action';
import { revalidatePath } from 'next/cache';
import { createArticleSchema } from './schema';
import { InputType, ReturnType } from './type';

const handler = async (
  input: InputType
): Promise<ReturnType> => {
  const res = await fetch(
    `http://127.0.0.1:5000/api/user/${input.user_id}/article`,
    {
      method: 'POST',
      headers: {
        Accept: 'application/json',
      },
    }
  );
  let data: string = await res.json();
  if (res.ok) {
    revalidatePath('/write');
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

export const createarticle = createSafeAction(
  createArticleSchema,
  handler
);
