import MobileFooter from "@/components/mobile_footer";
import Sidebar from "@/components/sidebar_desktop";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="container flex">
      <Sidebar />
      <main className="flex min-h-screen grow flex-col pb-96 ">{children}</main>
    </div>
  );
};

export default HomeLayout;
