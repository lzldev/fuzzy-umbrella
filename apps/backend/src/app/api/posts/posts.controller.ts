import { Controller, Inject, Post, UseGuards } from "@nestjs/common";
import { StorageService } from "~/app/storage/storage.service";
import { ClerkGuard } from "../auth/clerk/clerk.guard";

@Controller("posts")
@UseGuards(ClerkGuard)
export class PostsController {
  @Inject()
  private readonly storage: StorageService;
  @Post("create")
  sendPost() {
    return this.storage.presignedPost("mediathing-posts-57edd0f", "object.png");
  }

  @Post("test")
  testAuth() {
    return { data: "ok" };
  }
}
