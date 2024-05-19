import SessionProvider from "@/context/session_context";
import { Toaster } from "sonner";

const PlatformLayout = ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  return (
    <SessionProvider>
      {children}
      <Toaster />
    </SessionProvider>
  );
};

export default PlatformLayout;
