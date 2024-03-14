'use server';

import { createSafeAction } from '@/lib/create-safe-action';
import { signupSchema } from './schema';
import { InputType, ReturnType } from './type';

const handler = async (
  input: InputType
): Promise<ReturnType> => {
  const res = await fetch(
    'http://127.0.0.1:5000/api/auth/signup',
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(input),
    }
  );
  let data: string = await res.json();
  if (res.ok) {
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

export const signup = createSafeAction(
  signupSchema,
  handler
);
