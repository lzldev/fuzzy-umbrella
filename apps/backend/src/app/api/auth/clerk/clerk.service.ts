import { Injectable } from "@nestjs/common";
import { HttpErrorByCode } from "@nestjs/common/utils/http-error-by-code.util";
import { eq } from "drizzle-orm";
import { Database, DatabaseProvider } from "~/app/database/database.provider";
import { clerkIdtoUserIdKey } from "~/app/redis/redis.keys";
import { RedisClient, RedisClientProvider } from "~/app/redis/redis.provider";

const CLERK_ID_EXPIRATION = 24 * 60 * 60; // 1 Day in Seconds

@Injectable()
export class ClerkService {
  @Database()
  private database: DatabaseProvider;

  @RedisClient()
  private redis: RedisClientProvider;

  public async getUserIdFromClerkID(clerkId: string): Promise<number> {
    const redisKey = clerkIdtoUserIdKey(clerkId);
    const value = await this.redis.get(redisKey);

    if (value) {
      return parseInt(value);
    }

    const query = await this.database.query.users
      .findFirst({
        columns: {
          id: true,
        },
        where: (users) => eq(users.clerk_id, clerkId),
      })
      .catch(() => ({ id: null as null }));

    if (query.id === null) {
      throw HttpErrorByCode[403];
    }

    void this.redis.setex(redisKey, CLERK_ID_EXPIRATION, query.id);

    return query.id;
  }
}
