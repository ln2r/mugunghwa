import Elysia from "elysia";
import { bearer } from "@elysiajs/bearer";
import { jwt } from "@elysiajs/jwt";
import { env } from "cloudflare:workers";
import { AuthService } from "../auth/service.js";
import { WritingService } from "./service.js";

const writingService = new WritingService();
const authService = new AuthService();

export const writings = new Elysia({ prefix: "/writings" })
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
    .get("/:query", async ({ status, params: { query } }) => {
        const writing = await writingService.getWriting(query);

        if (!writing) {
            throw status(404, "Writing not found");
        }

        return writing;
    })
    .get("/", () => writingService.getWritings())
    .post(
        "/",
        async ({ body }) => {
            return writingService.add(body);
        },
        {
            async beforeHandle({ bearer, set, status, jwt }) {
                return authService.validateSession(bearer, set, status, jwt);
            },
        },
    )
    .patch(
        "/:id",
        async ({ status, body, params: { id } }) => {
            const res = await writingService.update(id, body);

            if (!res) {
                throw status(404, "Writing not found");
            }

            return res;
        },
        {
            async beforeHandle({ bearer, set, status, jwt }) {
                return authService.validateSession(bearer, set, status, jwt);
            },
        },
    )
    .delete(
        "/:id",
        async ({ status, params: { id } }) => {
            const res = await writingService.delete(id);

            if (!res) {
                throw status(404, "Writing not found");
            }

            return res;
        },
        {
            async beforeHandle({ bearer, set, status, jwt }) {
                return authService.validateSession(bearer, set, status, jwt);
            },
        },
    );
