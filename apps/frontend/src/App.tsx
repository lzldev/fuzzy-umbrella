import { useState } from "react";
import {
  RouterProvider,
  ErrorComponent,
  createRouter,
} from "@tanstack/react-router";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import { routeTree } from "./routeTree.gen";

const router = createRouter({
  routeTree,
  defaultPendingComponent: () => (
    <div className={`p-2 text-2xl`}>loading...</div>
  ),
  defaultErrorComponent: ({ error }) => <ErrorComponent error={error} />,
  context: {
    auth: undefined!, // We'll inject this when we render
  },
  defaultPreload: "intent",
});
function App() {
  return (
    <>
      <RouterProvider router={router} defaultPreload="intent" context={{}} />
    </>
  );
}

export default App;
