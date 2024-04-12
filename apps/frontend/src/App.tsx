import { RouterProvider, createRouter } from "@tanstack/react-router";
// Import the generated route tree
import { routeTree } from "./routeTree.gen";
import { ClerkProvider, useAuth } from "@clerk/clerk-react";

// Create a new router instance
const router = createRouter({ routeTree, context: { auth: undefined! } });

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

const PUBLISHABLE_KEY = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;

if (!PUBLISHABLE_KEY) {
  throw new Error("Missing Publishable Key");
}

function App() {
  return (
    <ClerkProvider publishableKey={PUBLISHABLE_KEY}>
      <AppRouter />
    </ClerkProvider>
  );
}

function AppRouter() {
  const auth = useAuth();
  return <RouterProvider router={router} context={{ auth }} />;
}
export default App;
