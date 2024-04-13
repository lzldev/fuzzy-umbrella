import { createParamDecorator, type ExecutionContext } from '@nestjs/common';

const knownCookies = ['__session'] as const;

type KnownCookies = (typeof knownCookies)[number];

export const Cookies = createParamDecorator<KnownCookies>(
  (data: KnownCookies | string, ctx: ExecutionContext) => {
    const request = ctx.switchToHttp().getRequest();
    return data ? request.cookies?.[data] : request.cookies;
  },
);

export type CookieRecord = Record<(typeof knownCookies)[number], string> &
  Record<string, string>;
