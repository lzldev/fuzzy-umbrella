import { Outlet, createRootRouteWithContext } from "@tanstack/react-router";

import { ClerkProvider } from "@clerk/clerk-react";

import { TanStackRouterDevtools } from "@tanstack/router-devtools";
// Import your publishable key
const PUBLISHABLE_KEY = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;

if (!PUBLISHABLE_KEY) {
  throw new Error("Missing Publishable Key");
}

export const Route = createRootRouteWithContext<{}>()({
  component: RootComponent,
});

function RootComponent() {
  return (
    <div>
      <ClerkProvider publishableKey={PUBLISHABLE_KEY}>
        <Outlet />
      </ClerkProvider>
      <TanStackRouterDevtools position="bottom-right" />
    </div>
  );
}
