import { Elysia } from "elysia";
import { node } from "@elysiajs/node";
import { CloudflareAdapter } from "elysia/adapter/cloudflare-worker";
import { works } from "./works/index.js";

export default new Elysia({
    adapter: CloudflareAdapter,
})
    .use(works)
    .get("/", () => "Hello Cloudflare Worker!")
    .compile();
