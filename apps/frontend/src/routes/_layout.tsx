import { createFileRoute, Outlet } from "@tanstack/react-router";

export const Route = createFileRoute("/_layout")({
  component: Layout,
});
function Layout() {
  return (
    <div>
      funny
      <Outlet />
    </div>
  );
}