import { createFileRoute } from "@tanstack/react-router";
import { FlexContainer } from "../components/FlexContainer";

export const Route = createFileRoute("/funny")({
  component: () => (
    <FlexContainer>
      <form
        className="flex flex-col gap-2"
        action="https://localhost:3000/form"
        method="post"
        encType="multipart/form-data"
      >
        <input type="text" name="text" />
        <input type="file" name="file" id="file" size={200} />
        <button type="submit">Submit</button>
      </form>
    </FlexContainer>
  ),
});
