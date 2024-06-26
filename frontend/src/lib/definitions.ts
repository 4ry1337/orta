import { z } from "zod";

export const SignUpFormSchema = z.object({
  username: z
    .string()
    .min(2, { message: "Username must be at least 2 characters long." })
    .max(30, {
      message: "Username must not be longer than 30 characters.",
    })
    .trim(),
  email: z.string().email({ message: "Please enter a valid email." }).trim(),
  password: z
    .string()
    .min(8, { message: "Be at least 8 characters long" })
    .regex(/[a-zA-Z]/, { message: "Contain at least one letter." })
    .regex(/[0-9]/, { message: "Contain at least one number." })
    .regex(/[^a-zA-Z0-9]/, {
      message: "Contain at least one special character.",
    })
    .trim(),
});
export const SignInFormSchema = z.object({
  email: z.string().email({ message: "Please enter a valid email." }).trim(),
  password: z
    .string()
    .min(8, { message: "Be at least 8 characters long" })
    .regex(/[a-zA-Z]/, { message: "Contain at least one letter." })
    .regex(/[0-9]/, { message: "Contain at least one number." })
    .regex(/[^a-zA-Z0-9]/, {
      message: "Contain at least one special character.",
    })
    .trim(),
});

export const CreateArticleSchema = z.object({
  title: z.string().trim(),
  description: z.string().trim().optional(),
});

export const UpdateArticleSchema = z.object({
  title: z.string().trim().optional(),
  description: z.string().trim().optional(),
});

export const SaveArticleSchema = z.object({
  content: z.string(),
});

export const CreateSeriesSchema = z.object({
  label: z.string().trim(),
});

export const UpdateSeriesSchema = z.object({
  label: z.string().trim().optional(),
  image: z.string().url().optional(),
});

export const CreateListSchema = z.object({
  label: z.string().trim(),
  visibility: z.string().trim().optional(),
});

export const UpdateUserFormSchema = z.object({
  username: z
    .string()
    .min(2, {
      message: "Username must be at least 2 characters.",
    })
    .max(30, {
      message: "Username must not be longer than 30 characters.",
    })
    .trim()
    .optional(),
  bio: z.string().max(160).optional(),
  image: z.string().url().optional(),
  urls: z.array(z.string().optional()).optional(),
});

export const UploadAssetFormSchema = z.object({
  files: z.array(z.instanceof(File)).nullable(),
});

export const CreateCommentSchema = z.object({
  content: z.string().min(1),
});
