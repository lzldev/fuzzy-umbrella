import {
  Body,
  Controller,
  HttpException,
  Inject,
  Post,
  UseGuards,
} from "@nestjs/common";
import { StorageService } from "~/app/storage/storage.service";
import { ClerkGuard } from "../auth/clerk/clerk.guard";
import { RedisClient, RedisClientProvider } from "~/app/redis/redis.provider";
import { v1 } from "uuid";

import { z } from "zod";
import { Database, DatabaseProvider } from "~/app/database/database.provider";
import { ClerkUserID } from "../auth/clerk/clerk.decorator";
import { users } from "@artspace/db";
import { eq } from "drizzle-orm";

export const newPostSchema = z.object({
  content: z.string().min(0).max(150),
  fileSize: z.number({ coerce: true }),
});

@Controller("posts")
@UseGuards(ClerkGuard)
export class PostsController {
  @Database()
  private readonly database: DatabaseProvider;

  @Inject()
  private readonly storage: StorageService;

  @RedisClient()
  private readonly redis: RedisClientProvider;

  @Post("create")
  async sendPost(@ClerkUserID() userId: string, @Body() body: any) {
    console.info("[create.body]", body);

    const data = newPostSchema.parse(body);

    const query = await this.database
      .select({
        id: users.id,
        clerk_id: users.clerk_id,
      })
      .from(users)
      .where(eq(users.clerk_id, userId))
      .limit(1)
      .execute();

    const user = query.at(0);

    if (!user) {
      throw new HttpException("User not found in database", 500);
    }

    const uuid = v1();

    const post = {
      id: uuid,
      content: data.content,
      userId: user.clerk_id,
    };

    const fileName = uuid;

    const presignedPost = await this.storage.presignedPost(
      "mediathing-posts-57edd0f", //TODO: GET THIS VALUE FROM ENV.
      fileName,
      {
        expected_file_size: data.fileSize,
      }
    );

    void this.redis.hset(`post:${uuid}`, post);

    return { fileName, presignedPost };
  }

  @Post("test")
  testAuth() {
    return { data: "ok" };
  }
}
