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
      <button
        className="p-2 text-white rounded-md bg-fuchsia-500 kjj"
        onClick={async () => {
          const req = await ofetch("http://localhost:8000/ws/ping/clerk", {
            method: "GET",
            credentials: "include",
          });

          console.log(req);

          setPosts(req.posts);
        }}
      >
        clerk
      </button>
      <button
        className="p-2 text-white rounded-md bg-fuchsia-500 kjj"
        onClick={async () => {
          const req = await ofetch("http://localhost:8000/ws/ping", {
            method: "GET",
            credentials: "include",
          });

          console.log(req);

          setPosts(req.posts);
        }}
      >
        Test
      </button>
      <Link to="/upload">
        <button className="p-2 text-white rounded-md bg-fuchsia-500 kjj">
          upload
        </button>
      </Link>
      <button
        className="p-2 text-white rounded-md bg-fuchsia-500 kjj"
        onClick={async () => {
          const res = await ofetch<{
            id: number;
            image_url: string;
            username: string;
            posts: Posts;
          }>("http://localhost:3000/api/profile", {
            method: "GET",
            credentials: "include",
          });

          console.log(res);

          setPosts(res.posts);
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
            <div className="flex flex-col p-2 bg-neutral-200 gap-y-2">
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
