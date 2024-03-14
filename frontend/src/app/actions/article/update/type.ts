import { z } from 'zod';

import { ActionState } from '@/types';

import { Article } from '@prisma/client';
import { updateArticleSchema } from './schema';


export type UpdateArticleSchema = z.infer<typeof updateArticleSchema>
export type InputType = UpdateArticleSchema;
export type ReturnType = ActionState<InputType, Article>;
