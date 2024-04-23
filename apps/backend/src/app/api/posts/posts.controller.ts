import { Controller, Inject, Post, Req, UseGuards } from "@nestjs/common";
import { StorageService } from "~/app/storage/storage.service";
import { ClerkGuard } from "../auth/clerk/clerk.guard";
import {
  ClerkSession,
  ClerkSessionParam,
  ClerkUser,
  ClerkUserParam,
} from "../auth/clerk/clerk.decorator";

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
