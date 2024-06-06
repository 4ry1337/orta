import { ImageIcon } from "@radix-ui/react-icons";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Editor } from "@tiptap/react";

const ImageButton = ({ editor }: { editor: Editor }) => {
  const [link, setlink] = useState("");
  return (
    <Dialog>
      <DialogTrigger>
        <Button variant={"ghost"} size={"icon"}>
          <ImageIcon />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>Add Youtube Video</DialogTitle>
        <DialogDescription>
          <Input
            onChange={(val) => setlink(val.target.value)}
            placeholder="Image URL"
          />
        </DialogDescription>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="secondary">
              Close
            </Button>
          </DialogClose>
          <DialogClose asChild>
            <Button
              onClick={() => {
                editor.commands.setImage({
                  src: link,
                });
              }}
            >
              Add
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default ImageButton;
