'use client'

import { updateArticleSchema } from "@/app/actions/article/update/schema";
import { UpdateArticleSchema } from "@/app/actions/article/update/type";
import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem } from "@/components/ui/command";
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Popover, PopoverTrigger } from "@/components/ui/popover";
import { Separator } from "@/components/ui/separator";
import { toast } from "@/components/ui/use-toast";
import { cn } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { PopoverContent } from "@radix-ui/react-popover";
import { format } from "date-fns";
import { CalendarIcon, CaretSortIcon, CheckIcon } from "@radix-ui/react-icons";
import { useForm } from "react-hook-form";
import { Input } from "@/components/ui/input";
import { Article } from "@prisma/client";
import { HTMLAttributes, useState } from "react";
import { useAction } from "@/hooks/useAction";
import { updatearticle } from "@/app/actions/article/update";
import { languages } from "@/lib/languages";
import { Badge } from "@/components/ui/badge";
import { TagInput } from "@/components/tags/TagInput";
import SortableList from "react-easy-sort";

interface articleSettingsProps extends HTMLAttributes<HTMLDivElement> {
  article: Article,
}
// languages.filter(language =>
//         article.tag_list.includes(language.value)
//       )[0].value ||

const ArticleSettings = ({ article }: articleSettingsProps) => {
  const ArticleForm = useForm<UpdateArticleSchema>({
    resolver: zodResolver(updateArticleSchema),
    defaultValues: {
      id: article.id,
      title: article.title || '',
      language:  'en',
      tag_list: article.tag_list,
    },
  })

  const { execute } = useAction(updatearticle, {
    onError: (error) => {
      toast({
        variant: 'destructive',
        title: error.status,
        description: error.message,
      });
    },
    onSuccess() {
      toast({
        title: 'Update'
      });
    },
  });

  function onSubmit(values: UpdateArticleSchema) {
    execute(values)
  }

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium">Article</h3>
        <p className="text-sm text-muted-foreground">
          Update your article settings. Set your preferred language and
          tags.
        </p>
      </div>
      <Separator />
      <Form {...ArticleForm}>
        <form onSubmit={ArticleForm.handleSubmit(onSubmit)} className="space-y-8">
          <FormField
            control={ArticleForm.control}
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
            control={ArticleForm.control}
            name="language"
            render={({ field }) => (
              <FormItem className="flex flex-col">
                <FormLabel>Language</FormLabel>
                <Popover>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant="outline"
                        role="combobox"
                        className={cn(
                          "w-[200px] justify-between",
                          !field.value && "text-muted-foreground"
                        )}
                      >
                        {field.value
                          ? languages.find(
                            (language) => language.value === field.value
                          )?.label
                          : "Select language"}
                        <CaretSortIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent className="w-[200px] p-0">
                    <Command>
                      <CommandInput placeholder="Search language..." />
                      <CommandEmpty>No language found.</CommandEmpty>
                      <CommandGroup>
                        {languages.map((language) => (
                          <CommandItem
                            value={language.label}
                            key={language.value}
                            onSelect={() => {
                              ArticleForm.setValue("language", language.value)
                            }}
                          >
                            <CheckIcon
                              className={cn(
                                "mr-2 h-4 w-4",
                                language.value === field.value
                                  ? "opacity-100"
                                  : "opacity-0"
                              )}
                            />
                            {language.label}
                          </CommandItem>
                        ))}
                      </CommandGroup>
                    </Command>
                  </PopoverContent>
                </Popover>
                <FormDescription>
                  This is the article&apos;s language.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={ArticleForm.control}
            name="tag_list"
            render={({ field }) => (
              <FormItem className="flex flex-col">
                <FormLabel>Tags</FormLabel>
                <FormControl>
                  <div className="w-full flex gap-3">
                    <div className="rounded-md max-w-[450px] flex flex-wrap gap-2">
                    </div>
                  </div>
                </FormControl>
                <FormDescription>
                  This is the article&apos;s language.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          /><FormField
            control={ArticleForm.control}
            name="scheduled"
            render={({ field }) => (
              <FormItem className="flex flex-col">
                <FormLabel>Schedule Publication</FormLabel>
                <Popover>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant={"outline"}
                        className={cn(
                          "w-[240px] pl-3 text-left font-normal",
                          !field.value && "text-muted-foreground"
                        )}
                      >
                        {field.value ? (
                          format(field.value, "PPP")
                        ) : (
                          <span>Pick a date</span>
                        )}
                        <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent className="w-auto p-0" align="start">
                    <Calendar
                      className="rounded-md border bg-background"
                      mode="single"
                      selected={field.value}
                      onSelect={field.onChange}
                      disabled={(date) =>
                        date > new Date("2100-01-01") || date < new Date()
                      }
                      initialFocus
                    />
                  </PopoverContent>
                </Popover>
                <FormDescription>
                  Your date of birth is used to calculate your age.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Update Article</Button>
        </form>
      </Form>
    </div >
  )
}

export default ArticleSettings;
