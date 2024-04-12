import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth/profile")({
  component: () => <div>Hello /(auth)/_layout/profile!</div>,
});
