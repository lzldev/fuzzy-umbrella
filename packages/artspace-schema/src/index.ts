export * from "./posts";

import { posts, users, users2 } from "@artspace/db";
import { createSelectSchema } from "drizzle-zod";
import { z } from "zod";

export const SomethingSchema = z.object({
  hello: z.string(),
  world: z.string(),
});

export const testSchema = createSelectSchema(users2);
export const selectUsersSchema = createSelectSchema(users);
export const selectPostsSchema = createSelectSchema(posts);

export const profileSchema = selectUsersSchema
  .pick({
    id: true,
    username: true,
    image_url: true,
  })
  .extend({
    posts: z.array(selectPostsSchema),
  });

export type Profile = z.infer<typeof profileSchema>;
