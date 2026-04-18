import { env } from "cloudflare:workers";
import type { Users } from "./model.ts";
import { Snowflake } from "@theinternetfolks/snowflake";
import type { D1Return } from "../commons/db-returns.ts";

export class AuthService {
    private db;
    private kv;

    constructor() {
        this.db = env.palebride;
        this.kv = env.kv;
    }

    githubAuth() {
        const params = new URLSearchParams({
            client_id: env.GITHUB_CLIENT_ID,
            scope: "user:email",
        });

        return `https://github.com/login/oauth/authorize?${params}`;
    }

    async handleCallback(jwt, code: string): Promise<any> {
        const id = Snowflake.generate();

        const tokenFetch = await fetch(
            "https://github.com/login/oauth/access_token",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    Accept: "application/json",
                },
                body: JSON.stringify({
                    client_id: env.GITHUB_CLIENT_ID,
                    client_secret: env.GITHUB_CLIENT_SECRET,
                    code,
                }),
            },
        );

        const { access_token } = (await tokenFetch.json()) as any;
        const userFetch = await fetch("https://api.github.com/user", {
            headers: {
                Authorization: `Bearer ${access_token}`,
                "User-Agent": "mugunghwa",
            },
        });

        const githubUser = (await userFetch.json()) as any;

        if (!env.ALLOWED_USER_ID.split(",").includes(String(githubUser?.id))) {
            return;
        }

        const now = new Date().toISOString();
        await this.db
            .prepare(
                `
                INSERT INTO users(id, username, avatar, url, provider_id, created, updated)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT (provider_id) DO UPDATE SET updated = ?`,
            )
            .bind(
                id,
                githubUser.login,
                githubUser.avatar_url,
                githubUser.url,
                String(githubUser.id),
                now,
                now,
                now,
            )
            .run();

        const res: D1Return = await this.db
            .prepare(
                `
              SELECT *
              FROM users
              WHERE provider_id = ?;`,
            )
            .bind(String(githubUser.id))
            .run<Users>();

        const user = res.results[0];

        // making refresh token
        const jwtToken = await jwt.sign({
            id: user.id,
            provider_id: user.provider_id,
        });
        const sessionToken = Snowflake.generate();
        await this.kv.put(
            `session:${sessionToken}`,
            JSON.stringify({
                id: user.id,
                provider_id: user.provider_id,
            }),
            {
                expirationTtl: 60 * 60 * 24 * 7, // 7 days
            },
        );

        return {
            user: user,
            accessToken: jwtToken,
            refreshToken: sessionToken,
        };
    }
}
