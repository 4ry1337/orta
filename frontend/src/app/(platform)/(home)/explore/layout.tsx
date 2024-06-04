"use client";

import Aside from "@/components/aside";

const ExploreLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="flex">
      <div className="w-full">
        <div>{children}</div>
      </div>
      <Aside>
        <div>Search Filters</div>
      </Aside>
    </div>
  );
};

export default ExploreLayout;
