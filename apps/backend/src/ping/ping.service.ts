import { Injectable } from "@nestjs/common";

@Injectable()
export class PingService {
	private map: Map<number, string> = new Map();

	push(id: number): string {
		this.map.set(id, new Date().toISOString());
		return this.map.get(id);
	}
	get(id: number): string | undefined {
		return this.map.get(id);
	}
	entries() {
		return Array.from(this.map.entries());
	}
}
