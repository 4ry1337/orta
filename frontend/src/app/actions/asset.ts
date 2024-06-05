import { UploadAssetFormSchema } from "@/lib/definitions";
import { toast } from "sonner";
import { z } from "zod";

export async function upload_asset(
  values: z.infer<typeof UploadAssetFormSchema>,
): Promise<string | null> {
  const formData = new FormData();
  formData.append("asset", values.asset);
  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/assets`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${sessionStorage.getItem("session")}`,
      "Content-Type": "multipart/form-data",
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
