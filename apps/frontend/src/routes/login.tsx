import { createFileRoute } from "@tanstack/react-router";
import {
  SignedIn,
  SignedOut,
  SignInButton,
  UserButton,
  UserProfile,
} from "@clerk/clerk-react";

export const Route = createFileRoute("/login")({
  component: Login,
});

function Login() {
  return (
    <div>
      <SignedOut>
        <SignInButton />
      </SignedOut>
      <SignedIn>
        <UserButton showName />
        <UserProfile />
      </SignedIn>
    </div>
  );
}
