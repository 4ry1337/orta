'use client';

import { Button } from '@/components/ui/button';
import { Calendar } from '@/components/ui/calendar';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import {
  Popover,
  PopoverTrigger,
} from '@/components/ui/popover';
import { Separator } from '@/components/ui/separator';
import { languages } from '@/lib/languages';
import { cn } from '@/lib/utils';
import { Article } from '@prisma/client';
import {
  CalendarIcon,
  CaretSortIcon,
  CheckIcon,
} from '@radix-ui/react-icons';
import { PopoverContent } from '@radix-ui/react-popover';
import { format } from 'date-fns';
import { HTMLAttributes } from 'react';

interface articleSettingsProps
  extends HTMLAttributes<HTMLDivElement> {
  article: Article;
}

const ArticleSettings = ({
  article,
}: articleSettingsProps) => {
  return (
    <div className='space-y-6'>
      <div>
        <h3 className='text-lg font-medium'>Article</h3>
        <p className='text-sm text-muted-foreground'>
          Update your article settings. Set your preferred
          language and tags.
        </p>
      </div>
      <Separator />
      <Form {...ArticleForm}>
        <form
          onSubmit={ArticleForm.handleSubmit(onSubmit)}
          className='space-y-8'
        >
          <FormField
            control={ArticleForm.control}
            name='title'
            render={({ field }) => (
              <FormItem>
                <FormLabel>Title</FormLabel>
                <FormControl>
                  <Input
                    placeholder='Article Title'
                    {...field}
                  />
                </FormControl>
                <FormDescription>
                  This is the title that will be displayed
                  on this article.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={ArticleForm.control}
            name='language'
            render={({ field }) => (
              <FormItem className='flex flex-col'>
                <FormLabel>Language</FormLabel>
                <Popover>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant='outline'
                        role='combobox'
                        className={cn(
                          'w-[200px] justify-between',
                          !field.value &&
                          'text-muted-foreground'
                        )}
                      >
                        {field.value
                          ? languages.find(
                            (language) =>
                              language.value ===
                              field.value
                          )?.label
                          : 'Select language'}
                        <CaretSortIcon className='ml-2 h-4 w-4 shrink-0 opacity-50' />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent className='w-[200px] p-0'>
                    <Command>
                      <CommandInput placeholder='Search language...' />
                      <CommandEmpty>
                        No language found.
                      </CommandEmpty>
                      <CommandGroup>
                        {languages.map((language) => (
                          <CommandItem
                            value={language.label}
                            key={language.value}
                            onSelect={() => {
                              ArticleForm.setValue(
                                'language',
                                language.value
                              );
                            }}
                          >
                            <CheckIcon
                              className={cn(
                                'mr-2 h-4 w-4',
                                language.value ===
                                  field.value
                                  ? 'opacity-100'
                                  : 'opacity-0'
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
            name='tag_list'
            render={({ field }) => (
              <FormItem className='flex flex-col'>
                <FormLabel>Tags</FormLabel>
                <FormControl>
                  <div className='flex w-full gap-3'>
                    <div className='flex max-w-[450px] flex-wrap gap-2 rounded-md'></div>
                  </div>
                </FormControl>
                <FormDescription>
                  This is the article&apos;s language.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={ArticleForm.control}
            name='scheduled'
            render={({ field }) => (
              <FormItem className='flex flex-col'>
                <FormLabel>Schedule Publication</FormLabel>
                <Popover>
                  <PopoverTrigger asChild>
                    <FormControl>
                      <Button
                        variant={'outline'}
                        className={cn(
                          'w-[240px] pl-3 text-left font-normal',
                          !field.value &&
                          'text-muted-foreground'
                        )}
                      >
                        {field.value ? (
                          format(field.value, 'PPP')
                        ) : (
                          <span>Pick a date</span>
                        )}
                        <CalendarIcon className='ml-auto h-4 w-4 opacity-50' />
                      </Button>
                    </FormControl>
                  </PopoverTrigger>
                  <PopoverContent
                    className='w-auto p-0'
                    align='start'
                  >
                    <Calendar
                      className='rounded-md border bg-background'
                      mode='single'
                      selected={field.value}
                      onSelect={field.onChange}
                      disabled={(date) =>
                        date > new Date('2100-01-01') ||
                        date < new Date()
                      }
                      initialFocus
                    />
                  </PopoverContent>
                </Popover>
                <FormDescription>
                  Your date of birth is used to calculate
                  your age.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type='submit'>Update Article</Button>
        </form>
      </Form>
    </div>
  );
};

export default ArticleSettings;
