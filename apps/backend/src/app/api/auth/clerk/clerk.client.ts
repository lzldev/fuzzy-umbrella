import { type ClerkClient, createClerkClient } from "@clerk/clerk-sdk-node";
import { type FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { EnvProvider } from "~/app/config/env.provider";

export const ClerkClientProviderToken = "CLERK_CLIENT_PROVIDER";
export type ClerkClientProvider = ClerkClient;

export const ClerkClientProviderFactory: FactoryProvider<ClerkClientProvider> =
  {
    inject: [EnvProvider],
    provide: ClerkClientProviderToken,
    useFactory: (config: EnvProvider) => {
      return createClerkClient({
        secretKey: config.clerk_secret_key,
      });
    },
  };

export function ClerkClient() {
  return applyDecorators(Inject(ClerkClientProviderToken));
}
