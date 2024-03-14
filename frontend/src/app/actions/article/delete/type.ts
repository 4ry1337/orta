import { z } from 'zod';

import { ActionState } from '@/types';

import { deleteArticleSchema } from './schema';

export type InputType = z.infer<typeof deleteArticleSchema>;
export type ReturnType = ActionState<InputType, string>;
