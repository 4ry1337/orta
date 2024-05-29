import Sidebar from "@/components/sidebar_desktop";
import { TooltipProvider } from "@/components/ui/tooltip";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="container flex">
      <TooltipProvider delayDuration={0}>
        <Sidebar className="shrink-0" />
        <main className="min-h-screen border-x w-full">{children}</main>
      </TooltipProvider>
    </div>
  );
};

// <Separator className="min-h-screen" orientation="vertical" />
export default HomeLayout;
