import { ThemeProvider } from '@/components/theme-provider';
import { Toaster } from '@/components/ui/toaster';

const PlatformLayout = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <ThemeProvider
      attribute='class'
      defaultTheme='system'
      enableSystem={true}
    >
      {children}
      <Toaster />
    </ThemeProvider>
  );
};

export default PlatformLayout;
