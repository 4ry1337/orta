"use client";

import { UpdateUserFormSchema } from "@/lib/definitions";
import { User } from "@/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { HTMLAttributes, useTransition } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import {
  Form,
  FormControl,
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

interface ProfileFormProps extends HTMLAttributes<HTMLDivElement> {
  user: User;
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
    },
    mode: "onChange",
  });

  const onSubmit = async (values: z.infer<typeof UpdateUserFormSchema>) => {
    startTransition(async () => {
      await update_user(user.username, values);
      await update();
    });
  };

  // const UploadAssetForm = useForm<z.infer<typeof UploadAssetFormSchema>>({
  //   resolver: zodResolver(UploadAssetFormSchema),
  // });
  //
  // const onUploadSubmit = async (
  //   values: z.infer<typeof UploadAssetFormSchema>,
  // ) => {
  //   startTransition(async () => {
  //     const res = await upload_asset(values);
  //     if (!!res) {
  //       UpdateUserForm.setValue("image", res);
  //     }
  //   });
  // };

  return (
    <Form {...UpdateUserForm}>
      <form
        onSubmit={UpdateUserForm.handleSubmit(onSubmit)}
        className="space-y-8"
      >
        <div className="flex flex-row gap-4 items-center">
          <div className="relative">
            <Avatar className="w-20 h-20">
              <AvatarImage src={user.image} alt="@avatar" />
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
                <DialogFooter>
                  <DialogClose asChild>
                    <Button type="button" variant="secondary">
                      Cancel
                    </Button>
                  </DialogClose>
                  <DialogClose asChild>
                    <Button disabled={pending} type="submit">
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
//     onClick={() => append({ value: "" })}
//   >
//     Add URL
//   </Button>
// </div>

export default ProfileForm;
