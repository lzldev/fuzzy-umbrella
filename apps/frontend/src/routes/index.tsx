import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";
import viteLogo from "/vite.svg";
import reactLogo from "~/assets/react.svg";
import { FlexContainer } from "~/components/FlexContainer";

export const Route = createFileRoute("/")({
	component: Index,
});

function Index() {
	const [count, setCount] = useState(0);
	return (
		<FlexContainer>
			<div className="gap-8 flex flex-col items-center">
				<div className="w-full max-w-screen-xl flex justify-center">
					<a href="https://vitejs.dev" target="_blank" rel="noreferrer">
						<img src={viteLogo} className="size-40" alt="Vite logo" />
					</a>
					<a href="https://react.dev" target="_blank" rel="noreferrer">
						<img src={reactLogo} className="size-40" alt="React logo" />
					</a>
				</div>
				<h1>Vite + React</h1>
				<div className="flex-col flex items-center gap-4">
					<button
						type="button"
						className="ring-2 ring-black rounded-md background-white px-4 py-2 text-black"
						onClick={() => setCount((count) => count + 1)}
					>
						count is {count}
					</button>
					<p>
						Edit <code>src/App.tsx</code> and save to test HMR
					</p>
				</div>
				<p className="read-the-docs">
					Click on the Vite and React logos to learn more
				</p>
			</div>
		</FlexContainer>
	);
}
