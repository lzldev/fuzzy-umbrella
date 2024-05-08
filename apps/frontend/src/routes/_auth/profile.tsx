import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth/profile")({
  component: Profile,
});

function Profile() {
  return (
    <div>
      <div>Hello /(auth)/_layout/profile!</div>
      <button
        className="bg-fuchsia-500 p-2 rounded-md"
        onClick={async () => {
          const fet = await fetch("http://localhost:3000/api/ping", {
            method: "GET",
            credentials: "include",
          });

          console.log(fet);
        }}
      >
        Ping
      </button>
      <button
        className="bg-fuchsia-500 p-2 rounded-md"
        onClick={async () => {
          const fet = await fetch("http://localhost:3000/api/profile", {
            method: "GET",
            credentials: "include",
          });

          console.log(fet);
        }}
      >
        Fake
      </button>
    </div>
  );
}
