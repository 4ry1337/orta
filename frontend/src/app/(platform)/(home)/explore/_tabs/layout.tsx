"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { Search, XIcon } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

const TabLayout = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  return (
    <>
      <div className="relative h-32">
        <div className="sticky w-full p-4 space-y-4">
          <div className="relative">
            <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input placeholder="Search" className="px-8" />
            <XIcon className="absolute right-2 top-2.5 h-4 w-4 text-muted-foreground" />
          </div>
          <div className="grid grid-cols-2 gap-2">
            <Button
              variant={
                pathname === "/explore/tabs/foryou" ? "secondary" : "outline"
              }
              asChild
            >
              <Link href="/explore/tabs/foryou">For You</Link>
            </Button>
            <Button
              variant={
                pathname === "/explore/tabs/trending" ? "secondary" : "outline"
              }
              asChild
            >
              <Link href="/explore/tabs/trending">Trending</Link>
            </Button>
          </div>
        </div>
        <Separator orientation="horizontal" />
      </div>
      {children}
    </>
  );
};

export default TabLayout;
