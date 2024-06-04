import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar";
import Link from "next/link";
import React from "react";

const top_authors = [
  {
    username: "4ry1337",
    image: undefined,
    initials: "RY",
    name: "Rakhat Yskak",
  },
];

const MainLayout = ({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) => {
  return (
    <div className="flex">
      <div className="grow">{children}</div>
      <aside className="shrink-0 px-4 py-2 w-64 flex-col gap-4 hidden lg:flex">
        <div className="">
          <h2 className="mb-4">Popular Writers</h2>
          <div className="flex flex-col gap-2">
            {top_authors.map((top_author) => {
              return (
                <Link
                  href={`/${top_author.username}`}
                  key={top_author.username}
                  className="inline-flex flex-row items-center"
                >
                  <Avatar className="h-10 w-10">
                    <AvatarImage src={top_author.image} alt="@avatar" />
                    <AvatarFallback>{top_author.initials}</AvatarFallback>
                  </Avatar>
                  <div className="mx-4 grow">{top_author.name}</div>
                </Link>
              );
            })}
          </div>
        </div>
      </aside>
    </div>
  );
};

export default MainLayout;
