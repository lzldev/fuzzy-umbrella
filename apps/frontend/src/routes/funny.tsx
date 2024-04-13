import { createFileRoute } from "@tanstack/react-router";
import { ofetch } from "ofetch";
import { FlexContainer } from "~/components/FlexContainer";

export const Route = createFileRoute("/funny")({
	component: FileUploadForm,
});

function FileUploadForm() {
	return (
		<FlexContainer>
			<div className="flex flex-col gap-2">
				<form
					className="flex flex-col gap-2"
					action="http://localhost:3000/form"
					method="post"
					encType="multipart/form-data"
				>
					<input type="text" name="text" />
					<input type="file" name="file" id="file" size={200} />
					<button type="submit">Submit</button>
				</form>
				<div>
					<button
						type="button"
						onClick={async () => {
							await ofetch("http://localhost:3000/ping", {
								credentials: "include",
							});
						}}
					>
						Send
					</button>
				</div>
			</div>
		</FlexContainer>
	);
}
