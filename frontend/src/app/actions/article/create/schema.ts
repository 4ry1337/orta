import { z } from 'zod';

export const createArticleSchema = z.object({
  user_id: z.string().min(1, {
    message: 'User id is required.',
  }),
});
