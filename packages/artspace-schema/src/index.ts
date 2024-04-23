import { z } from "zod";

export const SomethingSchema = z.object({
  hello: z.string(),
  world: z.string(),
});
