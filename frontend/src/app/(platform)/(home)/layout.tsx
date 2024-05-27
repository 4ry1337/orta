import Sidebar from "@/components/sidebar_desktop";
import { Separator } from "@/components/ui/separator";
import { TooltipProvider } from "@/components/ui/tooltip";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="container">
      <TooltipProvider delayDuration={0}>
        <div className="flex border-x">
          <Sidebar />
          <main className="min-h-screen overflow-x-hidden p-4 grow">
            {children}
          </main>
        </div>
      </TooltipProvider>
    </div>
  );
};

// <Separator className="min-h-screen" orientation="vertical" />
export default HomeLayout;
