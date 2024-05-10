import { writeFile } from "fs/promises";

const req = await fetch("https://docs.svix.com/webhook-ips.json");
const svixips = JSON.stringify(await req.json());

await writeFile("./svixip.json", svixips);
