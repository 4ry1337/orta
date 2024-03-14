import { languages } from "@/lib/languages"
import db from "@/lib/prismadb"
import { z } from "zod"

export const updateArticleSchema = z.object({
  id: z.number({
    required_error: "ID is requried"
  }),
  title: z.string({
    required_error: "Title is requried"
  }),
  language: z.string({
    required_error: "Language is requried"
  }),
  scheduled: z.date().optional(),
  tag_list: z.string().array(),
})
