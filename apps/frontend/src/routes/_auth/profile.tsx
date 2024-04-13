import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_auth/profile")({
	component: Profile,
});

function Profile() {
	return <div>Hello /(auth)/_layout/profile!</div>;
}
