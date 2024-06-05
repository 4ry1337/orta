import { Separator } from "@/components/ui/separator";

const SettingsLayout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div>
      <div className="space-y-0.5 p-4">
        <h1>Settings</h1>
        <p className="text-muted-foreground">
          Manage your account and profile settings.
        </p>
      </div>
      <Separator />
      {children}
    </div>
  );
};

export default SettingsLayout;
