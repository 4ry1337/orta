'use server';

import { createSafeAction } from '@/lib/create-safe-action';
import { createArticleSchema } from './schema';
import { InputType, ReturnType } from './type';
import { revalidatePath } from 'next/cache';
import db from '@/lib/prismadb';
import { Article } from '@prisma/client';

const handler = async (
  input: InputType
): Promise<ReturnType> => {
  const article = await db.article.create({
    data: {
      userIds: [input.user_id],
    }
  }).catch(e => {
    return {
      error: {
        message: 'Creation Failed',
        status: '400'
      }
    }
  })
  revalidatePath(`/write`);
  return {
    data: article as Article,
  }
};

export const createarticle = createSafeAction(
  createArticleSchema,
  handler
);
