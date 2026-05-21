import { Elysia } from "elysia";
import { CloudflareAdapter } from "elysia/adapter/cloudflare-worker";
import { works } from "./works/index.js";
import { auth } from "./auth/index.js";
import { writings } from "./writings/index.js";
import { cors } from "@elysia/cors";
import { files } from "./files/index.js";

export default new Elysia({
    adapter: CloudflareAdapter,
})
    .use(cors())
    .onAfterHandle({ as: "global" }, ({ response, request, set }) => {
        // this just plainly return anything which are not raw
        // reponse object, so redirect stuff just get returned
        // normally
        if (response instanceof Response) return response;

        const message: Record<string, string> = {
            GET: "Fetched successfully",
            POST: "Processed successfully",
            PUT: "Updated successfully",
            PATCH: "Updated successfully",
            DELETE: "Deleted successfully",
        };

        return {
            statusCode: set.status ?? 200,
            time: new Date().toISOString(),
            message: message[request.method] ?? "Success",
            data: response,
        };
    })
    .onError(({ code, error, set }) => {
        console.error(error);
        return {
            statusCode: set.status ?? 500,
            time: new Date().toISOString(),
            message: error.message,
            data: null,
        };
    })
    .use(auth)
    .use(works)
    .use(writings)
    .use(files)
    .get("/", () => "Hello Cloudflare Worker!")
    .compile();
