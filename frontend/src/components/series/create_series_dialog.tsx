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
import { CreateSeriesSchema } from "@/lib/definitions";
import { create_series } from "@/app/actions/series";
import { mutate } from "swr";

const CreateSeriesDialog = () => {
  const [pending, startTransition] = useTransition();

  const CreateSeriesForm = useForm<z.infer<typeof CreateSeriesSchema>>({
    resolver: zodResolver(CreateSeriesSchema),
    defaultValues: {
      label: "My Series",
    },
  });

  const onSubmit = async (values: z.infer<typeof CreateSeriesSchema>) => {
    startTransition(async () => {
      await create_series(values);
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
            <PlusIcon className="h-7 w-7" />
            Create Series
          </h3>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Series</DialogTitle>
        </DialogHeader>
        <Form {...CreateSeriesForm}>
          <form
            id="create_series"
            className="grid grid-2 py-4"
            onSubmit={CreateSeriesForm.handleSubmit(onSubmit)}
          >
            <FormField
              control={CreateSeriesForm.control}
              name="label"
              render={({ field }) => (
                <FormItem className="grid grid-cols-4 items-center gap-4">
                  <FormLabel>Label</FormLabel>
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
          <Button form="create_series" type="submit">
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default CreateSeriesDialog;
