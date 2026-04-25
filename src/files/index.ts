import Elysia, { t } from "elysia";
import { bearer } from "@elysiajs/bearer";
import { jwt } from "@elysiajs/jwt";
import { env } from "cloudflare:workers";
import { FileService } from "./service.js";
import { AuthService } from "../auth/service.js";

const fileService = new FileService();
const authService = new AuthService();

export const files = new Elysia({ prefix: "/files" })
    .use(bearer())
    .use(
        jwt({
            name: "jwt",
            secret: env.JWT_SECRET,
            iss: "mugunghwa-cfw",
            exp: "2h",
        }),
    )
    .onError(({ error, set }) => {
        console.error(error);
        set.status = 500;
        return {
            error: error.message,
            time: new Date().toISOString(),
        };
    })
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
            body: t.Object({
                file: t.File(),
            }),
            beforeHandle: async ({ bearer, set, status, jwt }) => {
                return authService.validateSession(bearer, set, status, jwt);
            },
        },
    );
