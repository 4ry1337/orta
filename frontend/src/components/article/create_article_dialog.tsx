"use client";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { z } from "zod";
import { useTransition } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { CreateArticleSchema } from "@/lib/definitions";
import { create_article } from "@/app/actions/article";
import { redirect } from "next/navigation";
import { Pencil1Icon } from "@radix-ui/react-icons";
import { slugifier } from "@/lib/utils";

const CreateArticleDialog = () => {
  const [pending, startTransition] = useTransition();

  const CreateArticleForm = useForm<z.infer<typeof CreateArticleSchema>>({
    resolver: zodResolver(CreateArticleSchema),
    defaultValues: {
      title: "My Story",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateArticleSchema>) => {
    startTransition(async () => {
      const article = await create_article(values);
      if (article) {
        redirect(`/article/${slugifier(article.title)}-${article.id}`);
      }
    });
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button
          variant={"ghost"}
          className="w-full h-auto rounded-xl p-6 border text-card-foreground shadow border-dashed"
        >
          <h3 className="flex gap-2 justify-center items-center font-semibold leading-none tracking-tight text-muted-foreground">
            <Pencil1Icon className="h-7 w-7" />
            Create Article
          </h3>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Article</DialogTitle>
        </DialogHeader>
        <Form {...CreateArticleForm}>
          <form
            id="create_article"
            className="grid py-4"
            onSubmit={CreateArticleForm.handleSubmit(onSubmit)}
          >
            <FormField
              control={CreateArticleForm.control}
              name="title"
              render={({ field }) => (
                <FormItem className="grid grid-cols-4 items-center gap-4">
                  <FormLabel>Title</FormLabel>
                  <FormControl>
                    <Input {...field} className="col-span-3" />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="secondary">
              Cancel
            </Button>
          </DialogClose>
          <Button form="create_article" type="submit">
            Create Article
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default CreateArticleDialog;
