"use client";

import Aside from "@/components/aside";
import { Separator } from "@/components/ui/separator";
import UserTab from "@/components/user/user_tab";

const MainLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="flex">
      <div className="w-full">{children}</div>
      <Aside>
        <div className="flex p-4">
          <h3>User</h3>
        </div>
        <UserTab />
      </Aside>
    </div>
  );
};

export default MainLayout;
