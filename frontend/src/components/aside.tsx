import { HTMLAttributes } from "react";
import MarketingLinks from "./marketing_links";
import { cn } from "@/lib/utils";

interface AsideProps extends HTMLAttributes<HTMLDivElement> { }

const Aside = ({ children, ...props }: AsideProps) => {
  return (
    <aside className="hidden relative min-h-screen shrink-0 lg:block lg:w-96">
      <div
        className={cn(
          "fixed w-full h-full max-h-full border-l flex flex-col justify-between max-w-96",
          props.className,
        )}
      >
        <div className="flex p-4 pl-10 flex-col overflow-hidden space-y-4">
          {children}
        </div>
        <MarketingLinks />
      </div>
    </aside>
  );
};

export default Aside;
