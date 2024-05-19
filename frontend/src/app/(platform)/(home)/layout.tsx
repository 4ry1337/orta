import Sidebar from "@/components/sidebar_desktop";

const HomeLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="container flex">
      <Sidebar />
      <main className="min-h-screeng grow">{children}</main>
    </div>
  );
};

export default HomeLayout;
