'use server';

import { createSafeAction } from '@/lib/create-safe-action';
import { updateArticleSchema } from './schema';
import { InputType, ReturnType } from './type';
import { revalidatePath } from 'next/cache';
import db from '@/lib/prismadb';
import { Article } from '@prisma/client';

const handler = async (
  input: InputType
): Promise<ReturnType> => {
  const article = await db.article.update({
    where: {
      id: input.id,
    },
    data: {
      title: input.title
    }
  }).catch(e => {
    return {
      error: {
        message: 'Creation Failed',
        status: '400'
      }
    }
  })
  return {
    data: article as Article,
  }
};

export const updatearticle = createSafeAction(
  updateArticleSchema,
  handler
);
