import { Module } from "@nestjs/common";
import { APP_GUARD, RouterModule } from "@nestjs/core";
import { PingModule } from "~/app/api/ping/ping.module";
import { PostsModule } from "~/app/api/posts/posts.module";
import { AuthModule } from "./auth/auth.module";
import { ClerkGuard } from "./auth/clerk/clerk.guard";
import { UserModule } from "./user/user.module";

@Module({
  imports: [
    AuthModule,
    PingModule,
    PostsModule,
    UserModule,
    RouterModule.register([
      { path: "api", module: PingModule },
      { path: "api", module: PostsModule },
      { path: "api", module: UserModule },
    ]),
  ],
  providers: [
    {
      provide: APP_GUARD,
      useClass: ClerkGuard,
    },
  ],
})
export class ApiModule {}

/**

@ApiModule ? 
@RestModule({
path:'/api'
imports:[PostsModule,ApiModule]
})

becomes


@Module({
imports:[PostsModule,ApiModule,RouterModule.register({

})]
}) 

*/
