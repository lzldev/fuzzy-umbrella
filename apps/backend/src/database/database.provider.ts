import { type FactoryProvider, Inject, applyDecorators } from "@nestjs/common";
import { EnvProvider } from "~/config/env.provider";
import { createConnection } from "~/lib/db/connection";

export const DatabaseProviderToken = "DATABASE";

export const DatabaseProvider: FactoryProvider = {
	inject: [EnvProvider],
	provide: DatabaseProviderToken,
	useFactory: (config: EnvProvider) => {
		return createConnection(
			config.turso_connection_url,
			config.turso_auth_token,
		);
	},
};

export function Database() {
	return applyDecorators(Inject(DatabaseProviderToken));
}
