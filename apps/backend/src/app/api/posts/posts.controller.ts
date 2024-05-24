import { Body, Controller, HttpException, Inject, Post } from "@nestjs/common";
import { v1 } from "uuid";
import { RedisClient, RedisClientProvider } from "~/app/redis/redis.provider";
import { StorageService } from "~/app/storage/storage.service";

import { Database, DatabaseProvider } from "~/app/database/database.provider";
import { ClerkUserID } from "../auth/clerk/clerk.decorator";
//@ts-ignore
import { users } from "@artspace/db";
import { PreparedPost } from "artspace-shared";
import { eq } from "drizzle-orm";
//@ts-ignore
import { NewPostSchema } from "@artspace/schema";
import { createPostKey } from "~/app/redis/redis.keys";

@Controller("posts")
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

    const data = NewPostSchema.parse(body);

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

    const post: PreparedPost = {
      id: uuid,
      content: data.content,
      userId: user.id.toString(),
    };

    //TODO: Move keys into a service.
    void this.redis.set(createPostKey(uuid), JSON.stringify(post));

    const fileName = uuid;

    const presignedPost = await this.storage.presignedPost(
      "mediathing-posts-57edd0f", //TODO: GET THIS VALUE FROM ENV.
      fileName,
      {
        expected_file_size: data.fileSize,
      }
    );

    return { fileName, presignedPost };
  }

  @Post("test")
  testAuth() {
    return { data: "ok" };
  }
}
