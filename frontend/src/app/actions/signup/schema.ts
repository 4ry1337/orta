import { z } from 'zod';

export const signupSchema = z.object({
  email: z.string().email().min(1, {
    message: 'Email is required.',
  }),
  username: z.string().min(1, {
    message: 'Username is required.',
  }),
  password: z.string().min(1, {
    message: 'Password is required.',
  }),
});
