import {
  SignInButton,
  SignedIn,
  SignedOut,
  UserButton,
} from "@clerk/clerk-react";
import { Link } from "@tanstack/react-router";

export function Navbar() {
  return (
    <div className="flex min-h-12 max-h-12 items-center w-full overflow-y-auto bg-white p-2 gap-2 justify-between">
      <div className="flex">
        <Link className="hover:underline" to="/">
          Mediathing
        </Link>
      </div>
      <div className="flex items-center justify-center gap-2">
        <SignedIn>
          <Link className="hover:underline" to="/upload">
            post
          </Link>
          <Link className="hover:underline" to="/ws">
            ws
          </Link>
          <Link className="hover:underline" to="/profile">
            profile
          </Link>
        </SignedIn>
        <Profile />
      </div>
    </div>
  );
}

function Profile() {
  return (
    <>
      <SignedOut>
        <SignInButton mode="modal">
          <div className="hover:underline">Sign in</div>
        </SignInButton>
      </SignedOut>
      <SignedIn>
        <UserButton showName />
      </SignedIn>
    </>
  );
}
