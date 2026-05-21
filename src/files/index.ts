import Elysia, { t } from "elysia";
import { bearer } from "@elysiajs/bearer";
import { FileService } from "./service.js";
import { auth } from "../auth/index.js";

const fileService = new FileService();

export const files = new Elysia({ prefix: "/files" })
    .use(bearer())
    .use(auth)
    .decorate("fileService", fileService)
    .get(
        "/:key",
        async ({ params: { key } }) => {
            const res = await fileService.file(key);

            if (!res) return new Response("File not found", { status: 404 });

            return new Response(res.body, {
                headers: {
                    "Content-Type":
                        res.httpMetadata.contentType ??
                        "application/octet-stream",
                    "Content-Length": res.size.toString(),
                },
            });
        },
        {
            params: t.Object({
                key: t.String(),
            }),
        },
    )
    .get(
        "/",
        async ({ query }) => {
            return await fileService.files(query.query);
        },
        {
            isSignedIn: true,
            query: t.Object({
                query: t.Optional(t.String()),
            }),
        },
    )
    .post(
        "/",
        async ({ body }) => {
            return await fileService.upload(body);
        },
        {
            isSignedIn: true,
            body: t.Object({
                file: t.File(),
            }),
        },
    );
