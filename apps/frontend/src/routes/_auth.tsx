import { useAuth } from "@clerk/clerk-react";
import {
  Outlet,
  createFileRoute,
  redirect,
  useNavigate,
} from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/_auth")({
  beforeLoad({ context }) {
    console.log("beforeLoad", context);
    if (context.auth.isLoaded && !context.auth.isSignedIn) {
      throw redirect({
        to: "/",
      });
    }
  },
  loader({ context }) {
    console.log("Loader", context);
  },
  component: LayoutComponent,
});

function LayoutComponent() {
  const auth = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (auth.isLoaded && !auth.isSignedIn) {
      navigate({
        to: "/",
      });
    }
  }, [auth]);
  if (!auth.isLoaded)
    return (
      <div className="flex flex-1 items-center justify-center">Loading...</div>
    );
  if (!auth.isSignedIn) return <></>;

  return (
    <div>
      <div className="bg-red-500">funnny</div>
      <Outlet />
    </div>
  );
}
