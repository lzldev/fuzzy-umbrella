import type { Request, Response, NextFunction } from "express";
import {
  CallHandler,
  ExecutionContext,
  Injectable,
  NestInterceptor,
  NestMiddleware,
  UseInterceptors,
  applyDecorators,
  createParamDecorator,
} from "@nestjs/common";
import { ClerkClient, ClerkClientProvider } from "./clerk.client";
import { Observable } from "rxjs";

export const REQ_CLERK_SESSION_ID_KEY = "clerk_session_id";
export const REQ_CLERK_USER_ID_KEY = "clerk_user_id";

const REQ_CLERK_SESSION_KEY = "clerk_session";
const REQ_CLERK_USER_KEY = "clerk_user";

export const ClerkSessionID = createParamDecorator<string>(
  (_: unknown, context: ExecutionContext) => {
    return context.switchToHttp().getRequest<Request>()[
      REQ_CLERK_SESSION_ID_KEY
    ];
  }
);

export const ClerkUserID = createParamDecorator<string>(
  (_: unknown, context: ExecutionContext) => {
    return context.switchToHttp().getRequest<Request>()[REQ_CLERK_USER_ID_KEY];
  }
);

@Injectable()
class _ClerkSession implements NestInterceptor {
  @ClerkClient()
  clerkClient: ClerkClientProvider;

  async intercept(
    context: ExecutionContext,
    next: CallHandler<any>
  ): Promise<Observable<any>> {
    const req = context.switchToHttp().getRequest<Request>();
    const sessionId = req[REQ_CLERK_SESSION_ID_KEY];

    const session = await this.clerkClient.sessions.getSession(sessionId);
    req[REQ_CLERK_SESSION_KEY] = session;

    return next.handle();
  }
}

export function ClerkSession() {
  return applyDecorators(UseInterceptors(_ClerkSession));
}

export const ClerkSessionParam = createParamDecorator(
  (_: unknown, context: ExecutionContext) => {
    return context.switchToHttp().getResponse()[REQ_CLERK_SESSION_KEY];
  }
);

@Injectable()
class _ClerkUser implements NestInterceptor {
  @ClerkClient()
  clerkClient: ClerkClientProvider;

  async intercept(
    context: ExecutionContext,
    next: CallHandler<any>
  ): Promise<Observable<any>> {
    const req = context.switchToHttp().getRequest<Request>();
    const userId = req[REQ_CLERK_USER_ID_KEY];

    const user = await this.clerkClient.users.getUser(userId);
    req[REQ_CLERK_USER_KEY] = user;

    return next.handle();
  }
}

export function ClerkUser() {
  return applyDecorators(UseInterceptors(_ClerkUser));
}

export const ClerkUserParam = createParamDecorator(
  (_: unknown, context: ExecutionContext) => {
    return context.switchToHttp().getRequest()[REQ_CLERK_USER_KEY];
  }
);
