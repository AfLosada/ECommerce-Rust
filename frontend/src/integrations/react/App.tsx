// This pragma is required so that React JSX is used instead of Qwik JSX
/** @jsxImportSource react */
import { qwikify$ } from "@builder.io/qwik-react";
import { ThemeProvider } from "~/integrations/react/theme-provider";
import { LoginForm } from "./login-form";

const App = () => {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <div className="flex justify-around h-lvh align-middle items-center">
        <LoginForm />
      </div>
    </ThemeProvider>
  );
};

export const QwikApp = qwikify$(App, { eagerness: "load" });

