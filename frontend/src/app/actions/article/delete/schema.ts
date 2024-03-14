import { z } from 'zod';

export const deleteArticleSchema = z.object({
  article_id: z.number(),
});
