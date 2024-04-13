import { Test, type TestingModule } from "@nestjs/testing";
import type { INestApplication } from "@nestjs/common";
import * as request from "supertest";
import { PingModule } from "~/ping/ping.module";

const setupModule = async () => {
	const moduleFixture: TestingModule = await Test.createTestingModule({
		imports: [PingModule],
	}).compile();

	const app = moduleFixture.createNestApplication();
	await app.init();
	return app;
};

describe("Ping Controller (e2e)", () => {
	describe("Should Get 0 Pings", () => {
		let app: INestApplication;
		beforeAll(async () => {
			app = await setupModule();
		});

		it("/ping (GET)", async () => {
			const r = await request(app.getHttpServer()).get("/ping").expect(200);

			expect(r.body).toMatchObject({
				data: [],
				time: expect.any(String),
			});
		});
	});

	describe("Should Create a Ping", () => {
		let app: INestApplication;
		beforeAll(async () => {
			app = await setupModule();
		});

		it("/ping/1 (POST)", async () => {
			const r = await request(app.getHttpServer()).post("/ping/1");

			const isoPattern = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$/;
			expect(r.body).toMatchObject({
				data: expect.stringMatching(isoPattern),
			});
		});

		it("/ping (GET) - Should Return New Ping", async () => {
			const r = await request(app.getHttpServer()).get("/ping").expect(200);

			expect(r.body.data.length).toBe(1);
		});
	});
});
