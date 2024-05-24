//@ts-ignore
import { users } from "@artspace/db";
//@ts-ignore
import { Profile } from "@artspace/schema";
import { Injectable } from "@nestjs/common";
import { eq } from "drizzle-orm";
import { Database, DatabaseProvider } from "~/app/database/database.provider";

@Injectable()
export class ProfileService {
  @Database()
  private database: DatabaseProvider;

  public async getUserProfile(userId: string) {
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

type A = Awaited<
  ReturnType<typeof ProfileService.prototype.getUserProfile>
> & {};
type B = Profile;

// type __test__Profile = Expect<Equal<A, B>>;
