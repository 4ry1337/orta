"use client";

import LogoIcon from "@/components/logo";
import { ModeToggle } from "@/components/theme/toggle";
import { Button } from "@/components/ui/button";
import Link from "next/link";
import {
  BellIcon,
  BookmarkIcon,
  HomeIcon,
  MagnifyingGlassIcon,
  Pencil1Icon,
  PersonIcon,
} from "@radix-ui/react-icons";
import UserButton from "./user/button";
import { HTMLAttributes } from "react";
import { useSession } from "@/context/session_context";

interface IRoute {
  label: string;
  icon: any;
  href: string;
}

const routes: IRoute[] = [
  {
    label: "Home",
    href: "/",
    icon: HomeIcon,
  },
  {
    label: "Search",
    href: "/search",
    icon: MagnifyingGlassIcon,
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
];

interface SidebarProps extends HTMLAttributes<HTMLDivElement> { }

const Sidebar = (props: SidebarProps) => {
  const { data, status } = useSession();

  return (
    <header
      id="sidebar"
      className="hidden h-screen shrink-0 px-4 py-2 md:flex xl:w-64"
    >
      <div className="flex w-full flex-col justify-between">
        <section className="flex flex-col justify-center gap-4">
          <div className="inline-flex w-full flex-col items-center justify-center gap-2 xl:flex-row xl:justify-normal">
            <Button
              variant={"ghost"}
              asChild
              className="h-14 rounded-full px-3.5 py-2 w-14 grow xl:w-auto xl:justify-start"
            >
              <Link href={"/"}>
                <LogoIcon className="h-7 w-7" />
                <h3 className="ml-2 hidden xl:block">Orta</h3>
              </Link>
            </Button>
            <ModeToggle
              variant={"ghost"}
              size={"icon"}
              className="rounded-full"
            />
          </div>
          <Button
            asChild
            className="h-14 rounded-full px-3.5 py-2 w-14 grow xl:w-auto xl:justify-start"
          >
            <Link href={"/write"} prefetch={false}>
              <Pencil1Icon className="h-7 w-7" />
              <h3 className="ml-2 hidden xl:block">Write</h3>
            </Link>
          </Button>
          <div className="flex flex-col gap-2">
            {routes.map((route) => {
              return (
                <Button
                  key={route.label}
                  variant={"ghost"}
                  asChild
                  className="h-14 rounded-full px-3.5 py-2 w-14 grow xl:w-auto xl:justify-start"
                >
                  <Link href={route.href}>
                    <route.icon className="h-7 w-7" />
                    <h3 className="ml-2  hidden xl:block">{route.label}</h3>
                  </Link>
                </Button>
              );
            })}
          </div>
        </section>
        <section className="flex flex-col justify-center gap-4">
          {status == "authenticated" && <UserButton user={data} />}
          {status == "unauthenticated" && (
            <Button
              asChild
              className="h-14 rounded-full px-3.5 py-2 w-14 grow xl:w-auto xl:justify-start"
            >
              <Link href={"/auth"}>
                <PersonIcon className="h-7 w-7" />
                <h3 className="ml-2 hidden xl:block">Sign In</h3>
              </Link>
            </Button>
          )}
        </section>
      </div>
    </header>
  );
};

export default Sidebar;