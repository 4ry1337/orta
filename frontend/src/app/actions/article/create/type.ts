import { z } from 'zod';

import { ActionState } from '@/types';

import { createArticleSchema } from './schema';
import { Article } from '@prisma/client';

export type InputType = z.infer<typeof createArticleSchema>;
export type ReturnType = ActionState<InputType, Article>;
