import { env } from "cloudflare:workers";
import { Snowflake } from "@theinternetfolks/snowflake";
import type { D1Return } from "../commons/db-returns.js";
import type { Writings } from "./model.js";

export class WritingService {
    private db;

    constructor() {
        this.db = env.palebride;
    }

    async getWritings() {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE deleted is NULL")
            .run<Writings>();

        return res.results;
    }

    async getWriting(id: string) {
        const res: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE id = ? AND deleted is NULL;")
            .bind(id)
            .run<Writings>();

        console.log(res);

        return res.results[0];
    }

    async add(body: Writings) {
        const id = Snowflake.generate();
        const now = new Date().toISOString();
        const slug = body.title.toLocaleLowerCase().replace(/\W/gi, "-");
        await this.db
            .prepare(
                `
                INSERT INTO writings (
                  id, title, slug, hero, body, created,
                  updated
                )
                VALUES
                  (?, ?, ?, ?, ?, ?, ?);

          `,
            )
            .bind(id, body.title, slug, body.hero, body.body, now, now)
            .run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE id = ?;")
            .bind(id)
            .run<Writings>();

        return res.results[0];
    }

    async update(id: string, body: Writings) {
        const exist: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE id = ?;")
            .bind(id)
            .run<Writings>();

        if (exist.results.length === 0) {
            return;
        }

        const slug = body.title.toLocaleLowerCase().replace(/\W/gi, "-");
        const now = new Date().toISOString();
        await this.db
            .prepare(
                `UPDATE
                  writings
                SET
                  title = ?,
                  slug = ?,
                  hero = ?,
                  body = ?,
                  updated = ?
                WHERE
                  id = ?;
            `,
            )
            .bind(id, body.title, slug, body.hero, body.body, now, id)
            .run();

        const res: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE id = ?;")
            .bind(id)
            .run<Writings>();

        return res.results[0];
    }

    async delete(id: string) {
        const exist: D1Return = await this.db
            .prepare("SELECT * FROM writings WHERE id = ?;")
            .bind(id)
            .run<Writings>();

        if (exist.results.length === 0) {
            return;
        }

        const now = new Date().toISOString();
        const res: D1Return = await this.db
            .prepare("UPDATE writings SET deleted = ? WHERE id = ?")
            .bind(now, id)
            .run();

        return res.success;
    }
}
