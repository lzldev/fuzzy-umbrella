import { createFileRoute } from "@tanstack/react-router";
import { ofetch } from "ofetch";
import { FlexContainer } from "~/components/FlexContainer";

export const Route = createFileRoute("/test")({
  component: FileUploadForm,
});

function FileUploadForm() {
  return (
    <FlexContainer>
      <div className="flex flex-col gap-2">
        <button
          onClick={async () => {
            const req = await ofetch("http://localhost:3000/api/posts/test", {
              method: "POST",
              credentials: "include",
            });
          }}
        />
      </div>
    </FlexContainer>
  );
}
