import type { Request } from "express";

import type { JwtPayload } from "@clerk/types";
import {
  CanActivate,
  ExecutionContext,
  Inject,
  Injectable,
} from "@nestjs/common";
import { ClerkClient, ClerkClientProvider } from "./clerk.client";
import { EnvProvider } from "~/app/config/env.provider";
import { createLocalJWKSet, jwtVerify } from "jose";
import { Reflector } from "@nestjs/core";
import { IS_PUBLIC_METADATA_KEY } from "../public.decorator";
import { ClerkJWTPayload } from "./clerk.types";
import { JWKS, JWKSProvider } from "../jwt/jwks.provider";

@Injectable()
export class ClerkGuard implements CanActivate {
  @Inject()
  reflector: Reflector;

  @Inject()
  env: EnvProvider;

  @JWKS()
  jwks: JWKSProvider;

  @ClerkClient()
  clerkClient: ClerkClientProvider;

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const isPublic = this.reflector.getAllAndOverride<boolean>(
      IS_PUBLIC_METADATA_KEY,
      [context.getHandler(), context.getClass()]
    );

    if (isPublic) return true;

    const req = context.switchToHttp().getRequest<Request>();
    const token = req.cookies["__session"];

    if (!token) {
      return false;
    }

    const verify = await jwtVerify<JwtPayload>(token, this.jwks).catch(
      (error) => ({
        error,
      })
    );

    if ("error" in verify) {
      return false;
    }

    verify.payload.sid;

    req["clerk_session_id"] = verify.payload.sid;
    req["clerk_user_id"] = verify.payload.sub;

    return true;
  }
}
