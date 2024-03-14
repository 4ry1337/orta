import { ThemeProvider } from '@/components/theme-provider';
import { Toaster } from '@/components/ui/toaster';
import AuthContext from '@/context/AuthContext';

const PlatformLayout = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <AuthContext>
      <ThemeProvider
        attribute='class'
        defaultTheme='system'
        enableSystem={true}
      >
        {children}
        <Toaster />
      </ThemeProvider>
    </AuthContext>
  );
};

export default PlatformLayout;
