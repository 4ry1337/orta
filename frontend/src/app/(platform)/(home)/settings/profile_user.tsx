"use client";

import {
  UpdateArticleSchema,
  UpdateUserFormSchema,
  UploadAssetFormSchema,
} from "@/lib/definitions";
import { FullUser } from "@/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { HTMLAttributes, useState, useTransition } from "react";
import { useFieldArray, useForm } from "react-hook-form";
import { z } from "zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { update_user } from "@/app/actions/user";
import { useSession } from "@/context/session_context";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { PlusIcon } from "@radix-ui/react-icons";
import { cn } from "@/lib/utils";
import { upload_asset } from "@/app/actions/asset";
import { DropzoneOptions } from "react-dropzone";
import {
  FileInput,
  FileUploader,
  FileUploaderContent,
  FileUploaderItem,
} from "@/components/ui/file_upload";
import Image from "next/image";
import { AspectRatio } from "@/components/ui/aspect-ratio";

interface ProfileFormProps extends HTMLAttributes<HTMLDivElement> {
  user: FullUser;
}

const ProfileForm = ({ user }: ProfileFormProps) => {
  const { update } = useSession({
    authenticated: true,
  });

  const [pending, startTransition] = useTransition();

  const UpdateUserForm = useForm<z.infer<typeof UpdateUserFormSchema>>({
    resolver: zodResolver(UpdateUserFormSchema),
    defaultValues: {
      username: user.username,
      bio: user.bio,
      urls: user.urls,
    },
    mode: "onChange",
  });

  // const { fields, append, remove } = useFieldArray<
  //   z.infer<typeof UpdateUserFormSchema>
  // >({
  //   name: "urls",
  //   control: UpdateUserForm.control,
  // });

  const onSubmit = async (values: z.infer<typeof UpdateUserFormSchema>) => {
    startTransition(async () => {
      await update_user(user.username, values);
      await update();
    });
  };

  const [user_image, setuser_image] = useState(user.image);

  const UploadAssetForm = useForm<z.infer<typeof UploadAssetFormSchema>>({
    resolver: zodResolver(UploadAssetFormSchema),
  });

  const dropzone = {
    accept: {
      "image/*": [".jpg", ".jpeg", ".png"],
    },
    multiple: false,
    maxFiles: 1,
    maxSize: 8 * 1024 * 1024,
  } satisfies DropzoneOptions;

  const onUploadSubmit = async (
    values: z.infer<typeof UploadAssetFormSchema>,
  ) => {
    startTransition(async () => {
      const res = await upload_asset(values);
      if (res) {
        setuser_image(res);
        await update_user(user.username, {
          image: res,
        });
        await update();
      }
    });
  };

  return (
    <Form {...UpdateUserForm}>
      <form
        onSubmit={UpdateUserForm.handleSubmit(onSubmit)}
        className="space-y-8"
      >
        <div className="flex flex-row gap-4 items-center">
          <div className="relative">
            <Avatar className="w-20 h-20">
              <AvatarImage
                src={
                  user_image && "http://localhost:5000/api/assets/" + user_image
                }
                className="object-cover"
                alt="@avatar"
              />
              <AvatarFallback>{user.username.at(0)}</AvatarFallback>
            </Avatar>
            <Dialog>
              <DialogTrigger asChild>
                <Button
                  className="rounded-full absolute -right-1 -bottom-1"
                  size={"icon"}
                >
                  <PlusIcon />
                </Button>
              </DialogTrigger>
              <DialogContent className="flex flex-col gap-6">
                <DialogHeader>
                  <DialogTitle>Upload profile image</DialogTitle>
                </DialogHeader>
                <Form {...UploadAssetForm}>
                  <form
                    id="image"
                    onSubmit={UploadAssetForm.handleSubmit(onUploadSubmit)}
                  >
                    <FormField
                      control={UploadAssetForm.control}
                      name="files"
                      render={({ field }) => (
                        <FormItem>
                          <FileUploader
                            value={field.value}
                            onValueChange={field.onChange}
                            dropzoneOptions={dropzone}
                            reSelect
                          >
                            {field.value && field.value.length > 0 ? (
                              <FileUploaderContent className="aspect-square">
                                {field.value.map((file, i) => (
                                  <FileUploaderItem
                                    key={i}
                                    index={i}
                                    aria-roledescription={`file ${i + 1} containing ${file.name
                                      }`}
                                    className="w-full h-full"
                                  >
                                    <AspectRatio className="size-full">
                                      <Image
                                        src={URL.createObjectURL(file)}
                                        alt={file.name}
                                        className="object-cover rounded-md"
                                        fill
                                      />
                                    </AspectRatio>
                                  </FileUploaderItem>
                                ))}
                              </FileUploaderContent>
                            ) : (
                              <FileInput className="aspect-square border-dashed border">
                                <div className="w-full h-full flex justify-center items-center">
                                  <p className="text-gray-400">
                                    Drop files here
                                  </p>
                                </div>
                              </FileInput>
                            )}
                          </FileUploader>
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
                  <DialogClose asChild>
                    <Button disabled={pending} form="image" type="submit">
                      Upload
                    </Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
          <div className="grow">
            <FormField
              control={UpdateUserForm.control}
              name="username"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Username</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
        </div>
        <FormField
          control={UpdateUserForm.control}
          name="bio"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Bio</FormLabel>
              <FormControl>
                <Textarea
                  placeholder="Tell us a little bit about yourself. (Optional)"
                  className="resize-none"
                  {...field}
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button disabled={pending} type="submit">
          Update profile
        </Button>
      </form>
    </Form>
  );
};

// <div>
//   {fields.map((field, index) => (
//     <FormField
//       control={UpdateUserForm.control}
//       key={field.id}
//       name={`urls.${index}`}
//       render={({ field }) => (
//         <FormItem>
//           <FormLabel className={cn(index !== 0 && "sr-only")}>
//             URLs
//           </FormLabel>
//           <FormDescription className={cn(index !== 0 && "sr-only")}>
//             Add links to your website, blog, or social media profiles.
//           </FormDescription>
//           <FormControl>
//             <Input {...field} />
//           </FormControl>
//           <FormMessage />
//         </FormItem>
//       )}
//     />
//   ))}
//   <Button
//     type="button"
//     variant="outline"
//     size="sm"
//     className="mt-2"
//     onClick={() => append("")}
//   >
//     Add URL
//   </Button>
// </div>
export default ProfileForm;
