import { Injectable } from '@nestjs/common';

const envFields = {
  TURSO_CONNECTION_URL: 'turso_connection_url',
  TURSO_AUTH_TOKEN: 'turso_auth_token',
  CLERK_PUBLISHABLE_KEY: 'clerk_publishable_key',
  CLERK_SECRET_KEY: 'clerk_secret_key',
} as const satisfies Record<string, keyof EnvProvider>;

@Injectable()
export class EnvProvider {
  public readonly turso_connection_url: string;
  public readonly turso_auth_token: string;
  public readonly clerk_publishable_key: string;
  public readonly clerk_secret_key: string;

  constructor() {
    for (const [env, field] of Object.entries(envFields)) {
      if (!process.env[env]) {
        throw new Error(`[CONFIG] MISSING ENV VARIABLE: ${env}`);
      }
      this[field] = process.env[env];
    }
  }
}
