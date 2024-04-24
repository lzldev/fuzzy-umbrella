import z from "zod";

export const NewPostSchema = z.object({
  content: z.string().min(0).max(150),
  fileSize: z.number({ coerce: true }),
});

export type NewPost = z.output<typeof NewPostSchema>;
