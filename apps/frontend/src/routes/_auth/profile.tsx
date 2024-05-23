import { profileSchema } from "@artspace/schema";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import { api } from "~/lib/api";

export const Route = createFileRoute("/_auth/profile")({
  component: Profile,
});

function Profile() {
  const { data, isLoading, error } = useQuery({
    queryKey: ["profile"],
    queryFn: () => api("/profile").then(profileSchema.parseAsync),
  });

  if (isLoading || !data) {
    return <div>loading...</div>;
  }

  return (
    <div>
      <div className="flex items-center gap-8 px-4 py-12 select-none text-neutral-800 bg-neutral-300">
        <img className="rounded-full" src={data?.image_url ?? ""} width={64} />
        <div className="flex text-xl">{data.username}</div>
      </div>
      <div className="flex flex-wrap gap-2 p-4 justify-stretch">
        {!isLoading &&
          data &&
          data.posts?.map((post) => {
            return (
              <div
                key={post.id}
                className="flex flex-col self-stretch rounded-md select-none bg-neutral-300 text-neutral-900 gap-y-4"
              >
                <div className="p-1">
                  <div>{post.content}</div>
                  <div className="text-end text-opacity-70">
                    {post.createdAt}
                  </div>
                </div>
                <img
                  className="flex flex-grow object-contain object-center w-full h-full bg-neutral-800"
                  src={`http://d3cbnixg2yv7o.cloudfront.net/${post.id}_thumb_small.jpeg`}
                />
              </div>
            );
          })}
      </div>
    </div>
  );
}
