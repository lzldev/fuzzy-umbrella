import { Link, createFileRoute } from "@tanstack/react-router";
import { ofetch } from "ofetch";
import { useState } from "react";

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
  const [posts, setPosts] = useState<Posts>();
  return (
    <div>
      <Link to="/upload">
        <button className="bg-fuchsia-500 p-2 rounded-md text-white kjj">
          upload
        </button>
      </Link>
      <button
        className="bg-fuchsia-500 p-2 rounded-md text-white kjj"
        onClick={async () => {
          const fet = await ofetch<{
            id: number;
            image_url: string;
            username: string;
            posts: Posts;
          }>("http://localhost:3000/api/profile", {
            method: "GET",
            credentials: "include",
          });

          console.log(fet);

          setPosts(fet.posts);
        }}
      >
        fetch
      </button>
      <div className="flex">
        <div>{}</div>
      </div>
      <div className="flex gap-2">
        {posts?.map((post) => {
          return (
            <div className="flex flex-col bg-neutral-200 p-2 gap-y-2">
              <div>{post.content}</div>
              <img
                className="object-contain object-center flex w-full h-full bg-neutral-800"
                src={`http://d3cbnixg2yv7o.cloudfront.net/${post.id}_thumb_small.jpeg`}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
}
