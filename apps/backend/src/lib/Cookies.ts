import { createParamDecorator, ExecutionContext } from '@nestjs/common';

const known_cookies = ['__session'] as const;
type KnownCookies = (typeof known_cookies)[number];

export const Cookies = createParamDecorator<KnownCookies>(
  (data: KnownCookies | string, ctx: ExecutionContext) => {
    const request = ctx.switchToHttp().getRequest();
    return data ? request.cookies?.[data] : request.cookies;
  },
);

export type CookieRecord = Record<(typeof known_cookies)[number], string> &
  Record<string, string>;
