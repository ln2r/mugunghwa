import { env } from "cloudflare:workers";
import type { D1Return } from "../commons/db-returns.js";
import type { Files } from "./model.js";
import { Snowflake } from "@theinternetfolks/snowflake";

export class FileService {
    private db;
    private bucket;

    constructor() {
        this.db = env.palebride;
        this.bucket = env.bucket;
    }

    async files(query: string) {
        const res: D1Return = query
            ? await this.db
                  .prepare(
                      `SELECT * FROM files WHERE key LIKE ? AND deleted IS NULL;`,
                  )
                  .bind(query)
                  .run<Files>()
            : await this.db
                  .prepare(`SELECT * FROM files WHERE deleted IS NULL;`)
                  .run<Files>();

        return res.results;
    }

    async file(key: string) {
        const res: D1Return = await this.db
            .prepare(`SELECT * FROM files WHERE key = ? AND deleted IS NULL;`)
            .bind(key)
            .run<Files>();

        if (!res.results[0]) {
            return;
        }

        // get the file from the bucket
        return this.bucket.get(key);
    }

    async upload(body) {
        const bytes = new Uint8Array(await body.file.arrayBuffer());
        const id = Snowflake.generate();
        const now = new Date().toISOString();
        const fileName = `${id}-${body.file.name}`;

        await this.db
            .prepare(
                `
              INSERT INTO files(id, key, created, updated)
              VALUES
                (?, ?, ?, ?);
        `,
            )
            .bind(id, fileName, now, now)
            .run();

        await this.bucket.put(fileName, bytes, {
            httpMetadata: {
                contentType: body.file.type,
            },
        });

        const res: D1Return = await this.db
            .prepare(`SELECT * FROM files WHERE id = ?;`)
            .bind(id)
            .run<Files>();

        return res.results[0];
    }
}
