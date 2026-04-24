import { Elysia } from "elysia";
import { node } from "@elysiajs/node";
import { CloudflareAdapter } from "elysia/adapter/cloudflare-worker";
import { works } from "./works/index.js";
import { auth } from "./auth/index.js";
import { writings } from "./writings/index.js";

export default new Elysia({
    adapter: CloudflareAdapter,
})
    .use(auth)
    .use(works)
    .use(writings)
    .get("/", () => "Hello Cloudflare Worker!")
    .compile();
