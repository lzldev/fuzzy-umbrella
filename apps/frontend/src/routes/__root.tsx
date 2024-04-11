import { useState } from "react";
import {
  Link,
  Outlet,
  createRootRouteWithContext,
} from "@tanstack/react-router";

import { TanStackRouterDevtools } from "@tanstack/router-devtools";

export const Route = createRootRouteWithContext<{}>()({
  component: RootComponent,
});

function RootComponent() {
  const [count, setCount] = useState(0);
  return (
    <div>
      <Outlet />

      <TanStackRouterDevtools position="bottom-right" />
    </div>
  );
}
