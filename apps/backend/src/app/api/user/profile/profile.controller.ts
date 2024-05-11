import { Controller, Get, Inject, Param } from "@nestjs/common";
import { ProfileService } from "./profile.service";
import { Public } from "../../auth/public.decorator";
import {
  ClerkUser,
  ClerkUserID,
  ClerkUserParam,
} from "../../auth/clerk/clerk.decorator";
import { ClerkService } from "../../auth/clerk/clerk.service";
import { User } from "@clerk/clerk-sdk-node";

@Controller("profile")
export class ProfileController {
  @Inject()
  private profileService: ProfileService;

  @Inject()
  private clerkService: ClerkService;

  @Get("/")
  @ClerkUser()
  public async currentUser(
    @ClerkUserID() clerkUserId: string,
    @ClerkUserParam() user: User
  ) {
    console.log("private_metadata", user);

    const userId = await this.clerkService.getUserIdFromClerkID(clerkUserId);
    const profile = await this.profileService.getUserProfile(userId);

    return profile;
  }

  @Public()
  @Get("/:id")
  public userProfile(@Param("id") id: number) {
    return this.profileService.getUserProfile(id);
  }
}
