import { Controller, Get, Inject, Injectable, Param } from "@nestjs/common";
import { ProfileService } from "./profile.service";
import { Public } from "../../auth/public.decorator";
import { ClerkUser, ClerkUserParam } from "../../auth/clerk/clerk.decorator";
import { User } from "@clerk/clerk-sdk-node";

@Controller("profile")
export class ProfileController {
  @Inject()
  private profileService: ProfileService;

  @Get("/")
  @ClerkUser()
  public currentUser(@ClerkUserParam() clerkUser: User) {
    console.log(clerkUser);
    this.profileService.getUserProfile(1);
  }

  @Public()
  @Get("/:id")
  public userProfile(@Param("id") id: number) {
    return this.profileService.getUserProfile(id);
  }
}
