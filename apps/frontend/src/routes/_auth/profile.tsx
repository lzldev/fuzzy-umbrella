import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { ofetch } from "ofetch";
import { api } from "~/lib/api";
import { profileSchema } from "@artspace/schema";

export const Route = createFileRoute("/_auth/profile")({
  component: Profile,
});

type Posts = {
  id: string;
  content: string;
  imageKey: string;
  createdAt: string;
  userId: number;
}[];

function Profile() {
  const { data, isLoading, error } = useQuery({
    queryKey: ["profile"],
    queryFn: () => api("/profile").then(profileSchema.parseAsync),
  });

  return (
    <div>
      <button
        className="p-2 text-white rounded-md bg-fuchsia-500"
        onClick={async () => {
          await ofetch<{
            id: number;
            image_url: string;
            username: string;
            posts: Posts;
          }>("http://localhost:8000/ws/ping", {
            method: "GET",
            credentials: "include",
          });
        }}
      >
        fetch
      </button>
      <div className="flex gap-2">
        {!isLoading && error && (
          <div className="text-red-500">{error.message}</div>
        )}
        {!isLoading &&
          data &&
          data.posts?.map((post) => {
            return (
              <div
                key={post.id}
                className="flex flex-col p-2 bg-neutral-200 gap-y-2"
              >
                <div>{post.content}</div>
                <img
                  className="flex object-contain object-center w-full h-full bg-neutral-800"
                  src={`http://d3cbnixg2yv7o.cloudfront.net/${post.id}_thumb_small.jpeg`}
                />
              </div>
            );
          })}
      </div>
    </div>
  );
}
