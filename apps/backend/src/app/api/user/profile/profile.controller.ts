import { Controller, Get, Inject, Injectable, Param } from "@nestjs/common";
import { ProfileService } from "./profile.service";
import { Public } from "../../auth/public.decorator";

@Controller("profile")
export class ProfileController {
  @Inject()
  private profileService: ProfileService;

  @Get("/")
  public currentUser() {
    this.profileService.getUserProfile(1);
  }

  @Public()
  @Get("/:id")
  public userProfile(@Param("id") id: number) {
    return this.profileService.getUserProfile(id);
  }
}
