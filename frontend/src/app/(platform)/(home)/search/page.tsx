"use client";

import { get_user } from "@/app/actions/user";
import { Separator } from "@/components/ui/separator";
import { useSearchParams } from "next/navigation";
import { useState } from "react";
import useSWR from "swr";

const SearchPage = () => {
  const searchParams = useSearchParams();
  const [query, setQuery] = useState("");

  const { data: users } = useSWR(get_user);

  return (
    <div className="flex flex-row max-w-full">
      <div className="w-full">{users}</div>
      <aside className="relative hidden h-screen shrink-0 border-r sm:block lg:w-64">
        <div className="fixed w-full px-2 py-4 h-full border-l flex flex-col justify-between max-w-fit xl:max-w-64">
          <section className="flex flex-col justify-center gap-4">
            <h3>Search Filters</h3>
          </section>
        </div>
      </aside>
    </div>
  );
};

export default SearchPage;
