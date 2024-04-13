import "dotenv/config";
import { drizzle } from "drizzle-orm/libsql";
import { createClient } from "@libsql/client";

export function createConnection(connectingUrl: string, authToken: string) {
	const client = createClient({
		url: connectingUrl,
		authToken: authToken,
	});

	return drizzle(client);
}

export type Connection = ReturnType<typeof createConnection>;
