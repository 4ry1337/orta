import LogoIcon from "@/components/logo";
import AuthTabs from "./_components/auth_tabs";

const Auth = () => {
  return (
    <div className="h-screen grid lg:grid-cols-[1fr,45vw]">
      <div className="hidden flex-col justify-between bg-muted p-10 lg:flex">
        <div className="flex items-center text-lg font-medium">
          <LogoIcon />
          <span className="ml-2">Orta</span>
        </div>
        <blockquote className="space-y-2">
          <p className="text-lg">
            &ldquo;A platform for creative professionals to create articles and
            effectively interact with audiences online.&rdquo;
          </p>
          <footer className="text-sm">Orta Developers</footer>
        </blockquote>
      </div>
      <div className="py-10 lg:p-10">
        <div className="mx-auto max-w-96">
          <AuthTabs />
        </div>
      </div>
    </div>
  );
};

export default Auth;
