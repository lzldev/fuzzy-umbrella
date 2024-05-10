import { Controller, Get, Inject, Param } from "@nestjs/common";
import { ProfileService } from "./profile.service";
import { Public } from "../../auth/public.decorator";
import { ClerkUser, ClerkUserID } from "../../auth/clerk/clerk.decorator";
import { ClerkService } from "../../auth/clerk/clerk.service";

@Controller("profile")
export class ProfileController {
  @Inject()
  private profileService: ProfileService;

  @Inject()
  private clerkService: ClerkService;

  @Get("/")
  @ClerkUser()
  public async currentUser(@ClerkUserID() clerkUserId: string) {
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
