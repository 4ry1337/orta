import { UploadAssetFormSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { z } from "zod";

export async function upload_asset(
  values: z.infer<typeof UploadAssetFormSchema>,
): Promise<string | null> {
  console.log(values);
  const formData = new FormData();
  let file = values.files?.at(0);
  if (!file) return null;
  formData.append("asset", file, file.name);
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/assets`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      // redirect: "follow",
    },
    credentials: "include",
    body: formData,
  }).then(async (res) => {
    if (!res.ok) {
      toast.error(`${res.status} - ${await res.text()}`);
      return null;
    }
    return await res.text();
  });
}
