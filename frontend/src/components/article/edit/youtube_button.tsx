import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Editor } from "@tiptap/react";
import { YoutubeIcon } from "lucide-react";
import { useState } from "react";

const YoutubeButton = ({ editor }: { editor: Editor }) => {
  const [link, setlink] = useState("");
  return (
    <Dialog>
      <DialogTrigger>
        <YoutubeIcon />
      </DialogTrigger>
      <DialogContent>
        <DialogTitle>Add Youtube Video</DialogTitle>
        <DialogDescription>
          <Input
            onChange={(val) => setlink(val.target.value)}
            placeholder="https://youtube.com/..."
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
                console.log(link);
                editor.commands.setYoutubeVideo({
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

export default YoutubeButton;
