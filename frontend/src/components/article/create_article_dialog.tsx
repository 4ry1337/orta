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
import { PlusIcon } from "@radix-ui/react-icons";
import { CreateArticleSchema } from "@/lib/definitions";
import { create_article } from "@/app/actions/article";

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
      await create_article(values);
    });
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant={"secondary"}>
          <PlusIcon className="mr-2" /> Create Article
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
