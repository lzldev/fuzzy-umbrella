import { Module } from "@nestjs/common";
import { RouterModule } from "@nestjs/core";
import { PingModule } from "~/app/api/ping/ping.module";
import { PostsModule } from "~/app/api/posts/posts.module";

@Module({
  imports: [
    PingModule,
    PostsModule,
    RouterModule.register([
      { path: "api", module: PingModule },
      { path: "api", module: PostsModule },
    ]),
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
