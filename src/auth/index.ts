import Elysia from "elysia";
import { jwt } from "@elysiajs/jwt";
import { env } from "cloudflare:workers";
import { AuthService } from "./service.ts";

const authService = new AuthService();

export const auth = new Elysia({ prefix: "/auth" })
    .onError(({ error, set }) => {
        console.error(error);
        set.status = 500;
        return {
            error: error.message,
            time: new Date().toISOString(),
        };
    })
    .use(
        jwt({
            name: "jwt",
            secret: env.JWT_SECRET,
            iss: "mugunghwa-cfw",
            exp: "2h",
        }),
    )
    .get("/login", async ({ res }) => {
        const url = authService.githubAuth();

        return Response.redirect(url, 302);
    })
    .get("/oauth/callback", async ({ jwt, query, set }) => {
        if (!query.code) {
            set.status = 400;
            return {
                error: "Missing code",
                time: new Date().toISOString(),
            };
        }

        const res = await authService.handleCallback(jwt, query.code);

        if (!res) {
            set.status = 401;
            return {
                error: "Invalid user",
                time: new Date().toISOString(),
            };
        }

        return res;
    })
    .post("/refresh-token", async ({ jwt, body, set }) => {
        const res = await authService.refreshToken(jwt, body);

        if (!res) {
            set.status = 401;
            return {
                error: "Token expired",
                time: new Date().toISOString(),
            };
        }

        return res;
    });
