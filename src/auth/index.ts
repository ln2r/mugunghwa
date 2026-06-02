import Elysia from "elysia";
import { jwt } from "@elysiajs/jwt";
import { env } from "cloudflare:workers";
import { AuthService } from "./service.ts";
import bearer from "@elysiajs/bearer";

const authService = new AuthService();

export const auth = new Elysia({ prefix: "/auth" })
    .use(
        jwt({
            name: "jwt",
            secret: env.JWT_SECRET,
            iss: "mugunghwa-cfw",
            exp: "2h",
        }),
    )
    .use(bearer())
    .macro({
        isSignedIn: {
            async resolve({ bearer, status, jwt, set }) {
                if (!bearer) {
                    set.headers["WWW-Authenticate"] =
                        `Bearer realm='sign', error="invalid_request"`;

                    return status(401, {
                        statusCode: 401,
                        time: new Date().toISOString(),
                        message: "Unauthorized",
                        data: null,
                    });
                }

                const user = await jwt.verify(bearer);

                if (!user) {
                    set.headers["WWW-Authenticate"] =
                        `Bearer realm='sign', error="invalid_request"`;

                    return status(401, {
                        statusCode: 401,
                        time: new Date().toISOString(),
                        message: "Token expired",
                        data: null,
                    });
                }

                return user;
            },
        },
    })
    .get("/login", async () => {
        const url = authService.githubAuth();

        return Response.redirect(url, 302);
    })
    .get("/oauth/callback", async ({ jwt, query }) => {
        if (!query.code) {
            throw new Error("Missing code");
        }

        const res = await authService.handleCallback(jwt, query.code);

        if (!res) {
            throw new Error("Invalid user");
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
    })
    .get(
        "/me",
        async ({ user }) => {
            return { user };
        },
        {
            isSignedIn: true,
        },
    );
