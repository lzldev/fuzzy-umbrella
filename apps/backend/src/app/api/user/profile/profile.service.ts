import { users } from "@artspace/db";
import { Injectable } from "@nestjs/common";
import { Profile } from "@artspace/schema";
import { eq } from "drizzle-orm";
import { Database, DatabaseProvider } from "~/app/database/database.provider";

@Injectable()
export class ProfileService {
  @Database()
  private database: DatabaseProvider;

  public async getUserProfile(userId: number) {
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

type Expect<T extends true> = T;
type Equal<X, Y> =
  (<T>() => T extends X ? 1 : 2) extends <T>() => T extends Y ? 1 : 2
    ? true
    : false;

type A = Awaited<
  ReturnType<typeof ProfileService.prototype.getUserProfile>
> & {};
type B = Profile;

type __test__Profile = Expect<Equal<A, B>>;
