import { createFileRoute } from "@tanstack/react-router";
import { ofetch } from "ofetch";
import { useRef, useState } from "react";
import { FlexContainer } from "~/components/FlexContainer";

export type PostResponse = {
  fileName: string;
  presignedPost: PresignedPost;
};

export type PresignedPost = {
  url: string;
  fields: Record<string, string>;
};

export const Route = createFileRoute("/upload")({
  component: FileUploadForm,
});

function FileUploadForm() {
  const [content, setContent] = useState<string>("");
  const fileRef = useRef<HTMLInputElement>(null!);

  const submit = async () => {
    if (fileRef.current.files?.length === 0) {
      alert("pick a image to be uploaded");
      return;
    }

    const file = fileRef.current.files?.item(0)!;

    const preparePost = await ofetch<PostResponse>(
      "http://localhost:3000/api/posts/create",
      {
        method: "POST",
        credentials: "include",
        body: {
          fileSize: file.size,
          content,
        },
      }
    );

    const formData = new FormData();

    for (const [key, value] of Object.entries(
      preparePost.presignedPost.fields
    )) {
      formData.append(key, value);
    }

    const formFile = fileRef.current.files?.item(0)!;

    formData.append(
      "file",
      new File([formFile], preparePost.fileName, {
        type: formFile.type,
      })
    );

    await fetch(preparePost.presignedPost.url, {
      method: "POST",
      body: formData,
    }).then((uploadResponse) => {
      console.log(uploadResponse);
    });
  };

  return (
    <FlexContainer>
      <div className="flex flex-col gap-2">
        <input
          type="text"
          name="content"
          value={content}
          onChange={(e) => {
            const txt = e.currentTarget!.value;
            setContent(txt);
          }}
        />
        <input ref={fileRef} type="file" name="file" size={200} />
        <button
          className="p-1 text-white rounded-md bg-fuchsia-500 disabled:bg-fuchsia-300 hover:ring-1 ring-white"
          onClick={submit}
        >
          Submit
        </button>
      </div>
    </FlexContainer>
  );
}
