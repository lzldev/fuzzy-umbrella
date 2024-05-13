import { Controller, Get, Inject, Param, Post } from "@nestjs/common";
import { Public } from "../auth/public.decorator";

@Public()
@Controller("ping")
export class PingController {
  @Get("/")
  public ping() {
    return "pong";
  }
}
