"use client";

import { useTheme } from "next-themes";

import { Button, ButtonProps } from "@/components/ui/button";
import { MoonIcon, SunIcon } from "@radix-ui/react-icons";

export function ModeToggle({ ...props }: ButtonProps) {
  const { theme, setTheme } = useTheme();

  const handleToggle = () => {
    theme === "dark" ? setTheme("light") : setTheme("dark");
  };

  return (
    <Button {...props} onClick={() => handleToggle()}>
      <SunIcon className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <MoonIcon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
