import LogoIcon from "@/components/logo";
import { ModeToggle } from "@/components/theme/toggle";
import { Button } from "@/components/ui/button";
import UserButton from "@/components/user/button";
import Link from "next/link";
import {
  BellIcon,
  BookmarkFilledIcon,
  HomeIcon,
  MagnifyingGlassIcon,
  Pencil1Icon,
} from "@radix-ui/react-icons";
import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar";

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
    icon: BookmarkFilledIcon,
  },
];

const Sidebar = () => {
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
              className="w-14 grow rounded-full xl:w-auto xl:justify-start"
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
            className="w-14 rounded-full xl:w-auto xl:justify-start"
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
                  className="w-14 xl:w-auto xl:justify-start"
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
          <UserButton />
        </section>
      </div>
    </header>
  );
};

export default Sidebar;
