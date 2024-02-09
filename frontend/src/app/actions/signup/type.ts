import { z } from 'zod';

import { ActionState } from '@/types';

import { signupSchema } from './schema';

export type InputType = z.infer<typeof signupSchema>;
export type ReturnType = ActionState<InputType, string>;
