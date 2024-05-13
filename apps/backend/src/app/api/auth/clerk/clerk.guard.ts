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
import { jwtVerify } from "jose";
import { Reflector } from "@nestjs/core";
import { IS_PUBLIC_METADATA_KEY } from "../public.decorator";
import { JWKS, JWKSProvider } from "../jwt/jwks.provider";
import {
  REQ_CLERK_SESSION_ID_KEY,
  REQ_CLERK_USER_ID_KEY,
} from "./clerk.decorator";

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

    const req = context.switchToHttp().getRequest<Request<{ jadas: true }>>();
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

    req[REQ_CLERK_SESSION_ID_KEY] = verify.payload.sid;
    req[REQ_CLERK_USER_ID_KEY] = verify.payload.sub;

    return true;
  }
}
