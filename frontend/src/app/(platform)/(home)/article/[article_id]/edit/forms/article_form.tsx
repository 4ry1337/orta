"use client";

import { publish, update_article } from "@/app/actions/article";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { UpdateArticleSchema } from "@/lib/definitions";
import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect, useMemo, useState, useTransition } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useArticle } from "@/context/article_context";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

const ArticleForm = () => {
  const { article, update } = useArticle();

  const [pending, startTransition] = useTransition();

  const UpdateArticleForm = useForm<z.infer<typeof UpdateArticleSchema>>({
    resolver: zodResolver(UpdateArticleSchema),
    defaultValues: {
      title: article.title,
      description: article.description ?? "",
    },
  });

  const onSubmit = async (values: z.infer<typeof UpdateArticleSchema>) => {
    startTransition(async () => {
      const res = await update_article(article.id, values);
      update({ ...article, ...res });
    });
  };

  return (
    <div className="space-y-8">
      <Form {...UpdateArticleForm}>
        <form
          onSubmit={UpdateArticleForm.handleSubmit(onSubmit)}
          className="space-y-8"
        >
          <FormField
            control={UpdateArticleForm.control}
            name="title"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Title</FormLabel>
                <FormControl>
                  <Input placeholder="Article Title" {...field} />
                </FormControl>
                <FormDescription>
                  This is the title that will be displayed on this article.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={UpdateArticleForm.control}
            name="description"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Textarea
                    placeholder="Article description. (Optional)"
                    className="resize-none h-40"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <div className="flex flex-row justify-end">
            <Button disabled={pending} type="submit">
              Update Article
            </Button>
          </div>
        </form>
      </Form>
      <Dialog>
        <DialogTrigger asChild>
          <Button className="w-full">Publish</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Are you sure?</DialogTitle>
            <DialogDescription>Publish {article.title}</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <DialogClose asChild>
              <Button>Close</Button>
            </DialogClose>
            <DialogClose asChild>
              <Button
                variant={"destructive"}
                onClick={() => {
                  publish(article.id);
                }}
              >
                Publish
              </Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ArticleForm;
