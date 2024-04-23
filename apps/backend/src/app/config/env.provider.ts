import { Injectable } from "@nestjs/common";

const requiredFields = {
  AWS_REGION: "aws_region",
  AWS_ACCESS_KEY_ID: "aws_access_key_id",
  AWS_SECRET_ACCESS_KEY: "aws_secret_access_key",
  CORS_FRONTEND_ORIGIN: "cors_frontend_origin",
  TURSO_CONNECTION_URL: "turso_connection_url",
  TURSO_AUTH_TOKEN: "turso_auth_token",
  CLERK_PUBLISHABLE_KEY: "clerk_publishable_key",
  CLERK_SECRET_KEY: "clerk_secret_key",
} as const satisfies Record<string, keyof EnvProvider>;

const optionalFields = {
  UPLOAD_LOCATION: {
    defaultValue: "./out/",
    key: "upload_location",
  },
  MAX_UPLOAD_SIZE: {
    defaultValue: Math.floor(1024 * 1024 * 10).toString(),
    key: "max_upload_size",
  },
} as const satisfies Record<
  string,
  { key: keyof EnvProvider; defaultValue: unknown }
>;

@Injectable()
export class EnvProvider {
  public readonly aws_region: string;
  public readonly aws_access_key_id: string;
  public readonly aws_secret_access_key: string;

  public readonly turso_connection_url: string;
  public readonly turso_auth_token: string;
  public readonly clerk_publishable_key: string;
  public readonly clerk_secret_key: string;
  public readonly cors_frontend_origin: string;

  public readonly upload_location: string;
  public readonly max_upload_size: string;

  constructor() {
    for (const [env, field] of Object.entries(requiredFields)) {
      if (!process.env[env]) {
        throw new Error(`MISSING ENV VARIABLE: ${env}`);
      }
      this[field] = process.env[env];
    }

    for (const [env, field] of Object.entries(optionalFields)) {
      if (!process.env[env]) {
        this[field.key] = field.defaultValue;
        continue;
      }
      this[field.key] = process.env[env];
    }
  }
}
