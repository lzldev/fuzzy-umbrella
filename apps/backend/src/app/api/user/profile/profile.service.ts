import { users } from "@artspace/db";
import { Injectable } from "@nestjs/common";
import { eq } from "drizzle-orm";
import { Database, DatabaseProvider } from "~/app/database/database.provider";

@Injectable()
export class ProfileService {
  @Database()
  private database: DatabaseProvider;

  public getUserProfile(userId: number) {
    return this.database.query.users
      .findFirst({
        columns: {
          id: true,
          image_url: true,
          username: true,
        },
        where: eq(users.id, userId),
        with: {
          posts: true,
        },
      })
      .execute();
  }
}
