import { Module } from "@nestjs/common";
import { StorageModule } from "~/app/storage/storage.module";
import { PostsController } from "./posts.controller";
import { AuthModule } from "../auth/auth.module";

@Module({
  imports: [StorageModule, AuthModule],
  controllers: [PostsController],
})
export class PostsModule {}
