import { FactoryProvider, applyDecorators, Inject } from "@nestjs/common";
import { createLocalJWKSet, createRemoteJWKSet } from "jose";
import { EnvProvider } from "~/app/config/env.provider";

export const JWKSProviderToken = "JWKS_PROVIDER";
export type JWKSProvider = ReturnType<typeof createRemoteJWKSet>;

export const JWKSProviderFactory: FactoryProvider<JWKSProvider> = {
  inject: [EnvProvider],
  provide: JWKSProviderToken,
  useFactory: (env: EnvProvider) => {
    return createRemoteJWKSet(new URL(env.clerk_jwks_url));
  },
};

export function JWKS() {
  return applyDecorators(Inject(JWKSProviderToken));
}
