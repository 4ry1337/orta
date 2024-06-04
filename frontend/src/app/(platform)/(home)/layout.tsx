"use client";

import Sidebar from "@/components/sidebar_desktop";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useSession } from "@/context/session_context";
import { IRoute } from "@/lib/types";
import {
  BellIcon,
  BookmarkIcon,
  GearIcon,
  MagnifyingGlassIcon,
  PersonIcon,
} from "@radix-ui/react-icons";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  const { status, data } = useSession();
  const routes: IRoute[] = [
    {
      label: "Explore",
      href: "/explore",
      icon: MagnifyingGlassIcon,
    },
  ];
  if (status == "authenticated") {
    routes.push(
      ...[
        {
          label: "Profile",
          href: `/${data.username}`,
          icon: PersonIcon,
        },
        {
          label: "Notifications",
          href: "/notifications",
          icon: BellIcon,
        },
        {
          label: "Lists",
          href: "/lists",
          icon: BookmarkIcon,
        },
        {
          label: "Settings",
          href: "/settings",
          icon: GearIcon,
        },
      ],
    );
  }
  return (
    <div className="container flex">
      <TooltipProvider delayDuration={0}>
        <Sidebar
          routes={routes}
          className="shrink-0 hidden h-screen md:block w-[4.5rem] xl:w-64"
        />
        <main className="border-x min-h-screen w-full">{children}</main>
      </TooltipProvider>
    </div>
  );
};

// <Separator className="min-h-screen" orientation="vertical" />
export default HomeLayout;
