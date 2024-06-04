import { HTMLAttributes } from "react";
import MarketingLinks from "./marketing_links";

interface AsideProps extends HTMLAttributes<HTMLDivElement> { }

const Aside = ({ children }: AsideProps) => {
  return (
    <aside className="hidden relative min-h-screen shrink-0 lg:block lg:w-96">
      <div className="fixed pl-10 p-4 w-full h-full border-l flex flex-col justify-between max-w-96">
        <div>{children}</div>
        <MarketingLinks />
      </div>
    </aside>
  );
};

export default Aside;
