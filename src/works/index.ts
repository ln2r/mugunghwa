import Elysia from "elysia";
import { WorkService } from "./service.js";
import { bearer } from "@elysiajs/bearer";
import { jwt } from "@elysiajs/jwt";
import { env } from "cloudflare:workers";
import { AuthService } from "../auth/service.js";

const workService = new WorkService();
const authService = new AuthService();

export const works = new Elysia({ prefix: "/works" })
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
    .get("/:id", async ({ status, params: { id } }) => {
        const work = await workService.getWork(id);
        if (work.length === 0) {
            throw status(404, "Work not found");
        }

        return work[0];
    })
    .get("/", () => workService.getWorks())
    .post(
        "/",
        async ({ body }) => {
            return workService.add(body);
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
            const res = await workService.update(id, body);

            if (!res) {
                throw status(404, "Work not found");
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
            const res = await workService.delete(id);

            if (!res) {
                throw status(404, "Work not found");
            }

            return "Deleted";
        },
        {
            async beforeHandle({ bearer, set, status, jwt }) {
                return authService.validateSession(bearer, set, status, jwt);
            },
        },
    );
