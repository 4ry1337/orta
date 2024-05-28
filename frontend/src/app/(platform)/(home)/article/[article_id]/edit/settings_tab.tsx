"use client";

import { update_article } from "@/app/actions/article";
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
import { Separator } from "@/components/ui/separator";
import { UpdateArticleSchema } from "@/lib/definitions";
import { Article } from "@/lib/types";
import { slugifier } from "@/lib/utils";
import { zodResolver } from "@hookform/resolvers/zod";
import { redirect } from "next/navigation";
import { HTMLAttributes, useTransition } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";

interface ArticleSettingsProps extends HTMLAttributes<HTMLDivElement> {
  article: Article;
}

const ArticleSettingsTab = ({ article }: ArticleSettingsProps) => {
  const [pending, startTransition] = useTransition();
  const UpdateArticleForm = useForm<z.infer<typeof UpdateArticleSchema>>({
    resolver: zodResolver(UpdateArticleSchema),
    defaultValues: {
      title: article.title,
    },
  });

  const onSubmit = async (values: z.infer<typeof UpdateArticleSchema>) => {
    startTransition(async () => {
      const res = await update_article(article.id, values);
      if (res) redirect(`/article/${slugifier(res.title)}-${res.id}/edit`);
    });
  };

  return (
    <div className="max-w-lg mx-auto">
      <div className="space-y-6">
        <div>
          <h3 className="text-lg font-medium">Article</h3>
          <p className="text-sm text-muted-foreground">
            Update your article settings. Set your preferred language and tags.
          </p>
        </div>
        <Separator />
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
            <Button disabled={pending} type="submit">
              Update Article
            </Button>
          </form>
        </Form>
      </div>
    </div>
  );
};

export default ArticleSettingsTab;

// <FormField
//   control={UpdateArticleForm.control}
//   name="scheduled"
//   render={({ field }) => (
//     <FormItem className="flex flex-col">
//       <FormLabel>Schedule Publication</FormLabel>
//       <Popover>
//         <PopoverTrigger asChild>
//           <FormControl>
//             <Button
//               variant={"outline"}
//               className={cn(
//                 "w-[240px] pl-3 text-left font-normal",
//                 !field.value && "text-muted-foreground",
//               )}
//             >
//               {field.value ? (
//                 format(field.value, "PPP")
//               ) : (
//                 <span>Pick a date</span>
//               )}
//               <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
//             </Button>
//           </FormControl>
//         </PopoverTrigger>
//         <PopoverContent className="w-auto p-0" align="start">
//           <Calendar
//             className="rounded-md border bg-background"
//             mode="single"
//             selected={field.value}
//             onSelect={field.onChange}
//             disabled={(date) =>
//               date > new Date("2100-01-01") || date < new Date()
//             }
//             initialFocus
//           />
//         </PopoverContent>
//       </Popover>
//       <FormDescription>
//         Your date of birth is used to calculate your age.
//       </FormDescription>
//       <FormMessage />
//     </FormItem>
//   )}
// />
//
//
// <FormField
//   control={UpdateArticleForm.control}
//   name="tag_list"
//   render={({ field }) => (
//     <FormItem className="flex flex-col">
//       <FormLabel>Tags</FormLabel>
//       <FormControl>
//         <div className="flex w-full gap-3">
//           <div className="flex max-w-[450px] flex-wrap gap-2 rounded-md"></div>
//         </div>
//       </FormControl>
//       <FormDescription>
//         This is the article&apos;s language.
//       </FormDescription>
//       <FormMessage />
//     </FormItem>
//   )}
// />
//
//
// <FormField
//   control={UpdateArticleForm.control}
//   name="language"
//   render={({ field }) => (
//     <FormItem className="flex flex-col">
//       <FormLabel>Language</FormLabel>
//       <Popover>
//         <PopoverTrigger asChild>
//           <FormControl>
//             <Button
//               variant="outline"
//               role="combobox"
//               className={cn(
//                 "w-[200px] justify-between",
//                 !field.value && "text-muted-foreground",
//               )}
//             >
//               {field.value
//                 ? languages.find(
//                   (language) => language.value === field.value,
//                 )?.label
//                 : "Select language"}
//               <CaretSortIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
//             </Button>
//           </FormControl>
//         </PopoverTrigger>
//         <PopoverContent className="w-[200px] p-0">
//           <Command>
//             <CommandInput placeholder="Search language..." />
//             <CommandEmpty>No language found.</CommandEmpty>
//             <CommandGroup>
//               {languages.map((language) => (
//                 <CommandItem
//                   value={language.label}
//                   key={language.value}
//                   onSelect={() => {
//                     UpdateArticleForm.setValue(
//                       "language",
//                       language.value,
//                     );
//                   }}
//                 >
//                   <CheckIcon
//                     className={cn(
//                       "mr-2 h-4 w-4",
//                       language.value === field.value
//                         ? "opacity-100"
//                         : "opacity-0",
//                     )}
//                   />
//                   {language.label}
//                 </CommandItem>
//               ))}
//             </CommandGroup>
//           </Command>
//         </PopoverContent>
//       </Popover>
//       <FormDescription>
//         This is the article&apos;s language.
//       </FormDescription>
//       <FormMessage />
//     </FormItem>
//   )}
// />
