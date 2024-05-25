import Sidebar from "@/components/sidebar_desktop";
import { TooltipProvider } from "@/components/ui/tooltip";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="container">
      <TooltipProvider delayDuration={0}>
        <div className="flex border-x">
          <Sidebar />
          <main className="min-h-screen p-4 grow">{children}</main>
        </div>
      </TooltipProvider>
    </div>
  );
};

// <Separator className="min-h-screen" orientation="vertical" />
export default HomeLayout;
