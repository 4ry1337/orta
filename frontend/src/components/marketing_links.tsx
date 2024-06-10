import Link from "next/link";
import React from "react";

const marketing = [
  {
    href: "/tos",
    label: "Terms of Service",
  },
  {
    href: "/about",
    label: "About",
  },
  {
    href: "/privacy",
    label: "Privacy Policy",
  },
  {
    href: "/cookie",
    label: "Cookie Policy",
  },
];

const MarketingLinks = () => {
  return (
    <div className="inline-flex p-4 pl-10 flex-wrap gap-4">
      {marketing.map((marketing_page) => {
        return (
          <Link
            key={marketing_page.href}
            href={marketing_page.href}
            className="text-primary underline-offset-4 hover:underline text-sm leading-none font-medium"
          >
            {marketing_page.label}
          </Link>
        );
      })}
      <div className="text-sm leading-none font-medium">Orta Inc.</div>
    </div>
  );
};

export default MarketingLinks;
