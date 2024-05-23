import {
  Controller,
  Get,
  HttpException,
  HttpStatus,
  Inject,
  Param,
} from "@nestjs/common";
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

    return await this.profileService.getUserProfile(userId);
  }

  @Public()
  @Get("/:id")
  public async userProfile(@Param("id") id: number) {
    const user = await this.profileService.getUserProfile(id);

    if (!user) {
      throw new HttpException("NOT FOUND", HttpStatus.NOT_FOUND);
    }

    return user;
  }
}
